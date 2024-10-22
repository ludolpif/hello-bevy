/* Needs some cargo crates :
 * - anyhow
 * - ctrlc
 * - gst (package gstreamer, from gstreamer-rs.git)
 * - gst-audio (package gstreamer-audio, from gstreamer-rs.git)
 * - gst-video (package gstreamer-video, from gstreamer-rs.git)
 * - gstfallbackswitch (package gst-plugin-fallbackswitch, gst-plugins-rs.git)
 * - std::{fs,sync}
 * - xdg
 * May be run with : GST_DEBUG_DUMP_DOT_DIR=. cargo run --example fd-to-twitch
 */

// https://github.com/matthew1000/gstreamer-cheat-sheet/blob/master/rtmp.md

use anyhow::{anyhow, Context}; // You may use RUST_BACKTRACE=1 cargo run ... to get source line numbers
use gst::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// For main() return type, see:
// - https://doc.rust-lang.org/std/process/trait.Termination.html
// - https://docs.rs/anyhow/latest/anyhow/
fn main() -> Result<(), anyhow::Error> {
    let video_width = 1920; // px
    let video_height = 1080; // px
    let video_framerate = 30; // FPS
    let audio_channels = 2; // stereo
    let audio_samplerate = 48000; // Hz

    let pipeline_name = "unixfd2twich";
    let mainsrc_str =
        "videotestsrc pattern=ball flip=true is-live=true num-buffers=300 name=videosrc audiotestsrc wave=8 is-live=true name=audiosrc";
    //"unixfdsrc socket-path=/tmp/lbs0 name=videosrc audiotestsrc wave=8 name=audiosrc";
    // "pipewiresrc autoconnect=false automatic-eos=false client-name=to-twitch";
    let fallback_source_uri = "file:///home/ludolpif/demo.mkv";
    let fallback_timeout_f32 = 1.0; // seconds
    let audio_encoder_str = // bitrate in bit/s
        "avenc_aac bitrate=160000";
    let video_encoder_str = // bitrate in kbit/s
        "x264enc threads=0 tune=zerolatency cabac=1 bframes=2 ref=1 key-int-max=60 bitrate=6000";
    let rtmp_config_file = "rtmp_dest.txt";
    //let rtmp_extra_str = ""; // REAL STREAM, USE WITH CAUTION
    let rtmp_extra_str = "?bandwidthtest=true"; // see results on https://inspector.twitch.tv

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Ctrl-C set running to false");
    })
    .context("Error setting Ctrl-C handler")?;

    let xdg_dirs = xdg::BaseDirectories::with_prefix("lbs")?;
    let rtmp_dest_path = xdg_dirs
        .place_config_file(rtmp_config_file)
        .context("cannot create configuration directory")?;
    let rtmp_dest_path_str = rtmp_dest_path.display().to_string();
    let rtmp_dest_read = std::fs::read_to_string(rtmp_dest_path)
        .context(format!("Cannot read {rtmp_dest_path_str}"))?;
    let rtmp_dest_str = rtmp_dest_read.trim(); // Strip plausible carriage return

    gst::init()?;
    gstfallbackswitch::plugin_register_static()?;

    let audio_caps_raw = gst_audio::AudioCapsBuilder::new()
        .format(gst_audio::AudioFormat::F32le)
        .rate(audio_samplerate)
        .channels(audio_channels)
        .channel_mask(3) //XXX https://gstreamer.freedesktop.org/documentation/audio/gstaudiochannels.html?gi-language=c#gst_audio_channel_positions_to_mask
        .layout(gst_audio::AudioLayout::Interleaved)
        .build();
    let audio_caps_encoded = gst_audio::AudioCapsBuilder::for_encoding("audio/mpeg")
        .field("mpegversion", 4)
        .build();
    let video_caps_raw = gst_video::VideoCapsBuilder::new()
        .format(gst_video::VideoFormat::Nv12)
        .width(video_width)
        .height(video_height)
        .framerate(gst::Fraction::from_integer(video_framerate))
        .build();
    let video_caps_encoded = gst_video::VideoCapsBuilder::for_encoding("video/x-h264")
        .field("profile", "high")
        .build();

    let pipeline = create_pipeline(
        pipeline_name,
        mainsrc_str,
        fallback_source_uri,
        gst::ClockTime::from_seconds_f32(fallback_timeout_f32),
        audio_caps_raw,
        audio_encoder_str,
        audio_caps_encoded,
        video_caps_raw,
        video_encoder_str,
        video_caps_encoded,
        rtmp_dest_str,
        rtmp_extra_str,
    )?;


    let bus = pipeline.bus().ok_or(anyhow!(
        "pipeline without bus, may be invalid or incomplete"
    ))?;

    println!("before PLAYING");
    pipeline.debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pipeline-before-play");

    pipeline
        .set_state(gst::State::Playing)
        .context("Failed to set_state Playing on pipeline")?;
    println!("started PLAYING");

    pipeline.debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pipeline-playing");
    println!("dumped PLAYING");

    let mut one_second_dumped = false;
    while running.load(Ordering::SeqCst) {
        // GStreamer pipeline event loop
        for msg in bus.iter_timed(gst::ClockTime::SECOND) {
            match msg.view() {
                gst::MessageView::Eos(eos) => {
                    println!("End Of Stream received {eos:?}");
                    break;
                }
                gst::MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    break;
                }
                _ => {
                    println!("{msg:?}");
                },
            }
        }
        if !one_second_dumped {
            pipeline.debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pipeline-after-one-sec");
            one_second_dumped = true;
        }
    }

    // Gracefull stop
    pipeline.debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pipeline-before-stop");
    pipeline
        .set_state(gst::State::Null)
        .context("Failed to stop the pipeline")?;

    Ok(())
}

fn create_pipeline(
    name: &str,
    mainsrc_str: &str,
    fallback_uri: &str,
    fallback_timeout: gst::ClockTime,
    audio_caps_raw: gst::caps::Caps,
    audio_encoder_str: &str,
    audio_caps_encoded: gst::caps::Caps,
    video_caps_raw: gst::caps::Caps,
    video_encoder_str: &str,
    video_caps_encoded: gst::caps::Caps,
    rtmp_dest_str: &str,
    rtmp_extra_str: &str,
) -> Result<gst::Pipeline, anyhow::Error> {
    let pipeline = gst::Pipeline::with_name(&name);

    let mainsrc_bin = parse_gst_bin(mainsrc_str, "mainsrc_bin", false)?;
    let mainsrc_videosrc = mainsrc_bin
        .by_name("videosrc")
        .context("Element named videosrc in mainsrc_str bin not found")?;
    let mainsrc_videosrc_srcpad = mainsrc_videosrc
        .static_pad("src")
        .expect("Static pad src not found in mainsrc videosrc");
    let mainsrc_bin_proxypad_video_builder =
        gst::GhostPad::builder_with_target(&mainsrc_videosrc_srcpad)?;
    let mainsrc_bin_proxypad_video = mainsrc_bin_proxypad_video_builder.name("video").build();
    mainsrc_bin.add_pad(&mainsrc_bin_proxypad_video)?;

    let mainsrc_audiosrc = mainsrc_bin
        .by_name("audiosrc")
        .context("Element named audiosrc in mainsrc bin not found")?;
    let mainsrc_audiosrc_srcpad = mainsrc_audiosrc
        .static_pad("src")
        .expect("Static pad src not found in mainsrc audiosrc");
    let mainsrc_bin_proxypad_audio_builder =
        gst::GhostPad::builder_with_target(&mainsrc_audiosrc_srcpad)?;
    let mainsrc_bin_proxypad_audio = mainsrc_bin_proxypad_audio_builder.name("audio").build();
    mainsrc_bin.add_pad(&mainsrc_bin_proxypad_audio)?;

    let mainsrc = mainsrc_bin.upcast::<gst::Element>();

    let fallbacksrc = gst::ElementFactory::make("fallbacksrc")
        //.property("enable-audio", false)
        .property("fallback-uri", fallback_uri)
        .property("restart-on-eos", true)
        .property("timeout", fallback_timeout)
        .property("source", mainsrc)
        .build()?;
    let audio_capsfilter_raw = gst::ElementFactory::make("capsfilter")
        .name("audio_capsfilter_raw")
        .property("caps", &audio_caps_raw)
        .build()?;
    let video_capsfilter_raw = gst::ElementFactory::make("capsfilter")
        .name("video_capsfilter_raw")
        .property("caps", &video_caps_raw)
        .build()?;
    let video_encoder =
        parse_gst_element(video_encoder_str).context("Failed to parse video_encoder element")?;
    let video_capsfilter_encoded = gst::ElementFactory::make("capsfilter")
        .name("video_capsfilter_encoded")
        .property("caps", &video_caps_encoded)
        .build()?;
    let video_parse = gst::ElementFactory::make("h264parse")
        .name("video_parse")
        .build()?;
    let video_queue = gst::ElementFactory::make("queue")
        .name("video_queue")
        .build()?;
    //let audioconvert = gst::ElementFactory::make("audioconvert").build()?;
    let audio_encoder =
        parse_gst_element(audio_encoder_str).context("Failed to parse audio_encoder element")?;
    let audio_capsfilter_encoded = gst::ElementFactory::make("capsfilter")
        .name("audio_capsfilter_encoded")
        .property("caps", &audio_caps_encoded)
        .build()?;
    let audio_parse = gst::ElementFactory::make("aacparse")
        .name("audio_parse")
        .build()?;
    let muxer = gst::ElementFactory::make("flvmux")
        .name("muxer")
        .property("streamable", true)
        .build()?;
    let decodebin = gst::ElementFactory::make("decodebin3")
        .property("async-handling", true)
        .build()?;
    let autoaudiosink = gst::ElementFactory::make("fakeaudiosink")
        .build()?;
    let autovideosink = gst::ElementFactory::make("autovideosink")
        .build()?;
    let playsink = gst::ElementFactory::make("playsink")
        .property("async-handling", true)
        .property("audio-sink", &autoaudiosink)
        .property("video-sink", &autovideosink)
        .build()?;
    let rmtpsink = gst::ElementFactory::make("rtmpsink")
        .property(
            "location",
            format!("{rtmp_dest_str}{rtmp_extra_str} live=1"),
        )
        .build()?;

    pipeline.add_many([
        &fallbacksrc,
        &audio_capsfilter_raw,
        &video_capsfilter_raw,
        &video_encoder,
        &video_parse,
        &video_queue,
        &video_capsfilter_encoded,
        &audio_encoder,
        &audio_capsfilter_encoded,
        &audio_parse,
        &muxer,
        &decodebin,
        &playsink,
        //&rmtpsink,
    ])?;
    // Not linking fallbacksrc yet because it has "Sometimes" pads (not Always nor Request).
    // See tutorial 3 and 7 here: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/tree/main/tutorials/src/bin
    // On "Failed to link elements" error here, you may re-run with GST_DEBUG=4 cargo run ...
    gst::Element::link_many([&video_encoder, &video_capsfilter_encoded, &video_parse, &video_queue, &muxer])?;
    gst::Element::link_many([&audio_encoder, &audio_capsfilter_encoded, &audio_parse, &muxer])?;
    gst::Element::link_many([&muxer, &decodebin])?;
    //gst::Element::link_many([&muxer, &rmtpsink])?;

    // Register a pad-added signal handler to connect pads when they will be available
    // (when pipeline will switch to state Playing)
    fallbacksrc.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());
        /* XXX seem deaklock sometimes ?!?
        src.downcast_ref::<gst::Bin>()
            .unwrap()
            .debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pad-added");
        */
        if src_pad.name() == "video" {
            let sink_pad = video_capsfilter_raw
                .static_pad("sink")
                .expect("Failed to get static sink pad from video_capsfilter_raw");
            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Link failed to video_capsfilter_raw.");
            } else {
                println!("Link succeeded to video_capsfilter_raw.");
            }
            let res = video_capsfilter_raw.link(&video_encoder);
            if res.is_err() {
                println!("Link failed to video_encoder.");
            } else {
                println!("Link succeeded to video_encoder.");
            }
        } else if src_pad.name() == "audio" {
            let sink_pad = audio_capsfilter_raw
                .static_pad("sink")
                .expect("Failed to get static sink pad from audio_capsfilter_raw");
            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Link failed from fallbacksrc to audio_capsfilter_raw");
            } else {
                println!("Link succeeded from fallbacksrc to audio_capsfilter_raw");
            }
            let res = audio_capsfilter_raw.link(&audio_encoder);
            if res.is_err() {
                println!("Link failed to audio_encoder.");
            } else {
                println!("Link succeeded to audio_encoder.");
            }
        }
    });
    decodebin.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());
        /* XXX seem deaklock sometimes ?!?
        src.downcast_ref::<gst::Bin>()
            .unwrap()
            .debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pad-added");
        */        
        if src_pad.name().starts_with("video_") {
            let sink_pad = playsink
                .request_pad_simple("video_raw_sink")
                .expect("Failed to get on request video_raw_sink pad from playsink");
            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Link failed");
            } else {
                println!("Link succeeded");
            }
        }
        if src_pad.name().starts_with("audio_") {
            let sink_pad = playsink
                .request_pad_simple("audio_raw_sink")
                .expect("Failed to get on request audio_raw_sink pad from playsink");
            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Link failed");
            } else {
                println!("Link succeeded");
            }
        }
    });

    Ok(pipeline)
}

fn parse_gst_element(element_description: &str) -> Result<gst::Element, anyhow::Error> {
    let mut context = gst::ParseContext::new();
    match gst::parse::launch_full(
        &element_description,
        Some(&mut context),
        gst::ParseFlags::empty(),
    ) {
        Ok(element) => Ok(element),
        Err(err) => {
            if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                Err(anyhow::Error::new(err).context(format!(
                    "Missing gstreamer element: {:?}",
                    context.missing_elements()
                )))
            } else {
                Err(anyhow::Error::new(err)
                    .context(format!("Failed to parse element: {element_description}")))
            }
        }
    }
}

fn parse_gst_bin(
    bin_description: &str,
    bin_name: &str,
    ghost_unlinked_pads: bool,
) -> Result<gst::Bin, anyhow::Error> {
    let mut context = gst::ParseContext::new();
    match gst::parse::bin_from_description_with_name_full(
        &bin_description,
        ghost_unlinked_pads,
        &bin_name,
        Some(&mut context),
        gst::ParseFlags::empty(),
    ) {
        Ok(element) => Ok(element.downcast::<gst::Bin>().unwrap()), // rust for GST_BIN(element) C macro
        Err(err) => {
            if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                Err(anyhow::Error::new(err).context(format!(
                    "Missing gstreamer elements: {:?}",
                    context.missing_elements()
                )))
            } else {
                Err(anyhow::Error::new(err)
                    .context(format!("Failed to parse bin: {bin_description}")))
            }
        }
    }
    // ghost_unlinked_pads: Ghost pads on the bin for unlinked source or sink pads within the bin can automatically be created (but only a maximum of one ghost pad for each direction will be created; if you expect multiple unlinked source pads or multiple unlinked sink pads and want them all ghosted, you will have to create the ghost pads yourself).
    // An iterator of elements in a bin can be retrieved with GstBinExtManual::iterate_elements().
    // An iterator of all pads can be retrieved with ElementExtManual::iterate_pads() .
    // let res = src_pad.is_linked();
}
