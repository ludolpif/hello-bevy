/* Needs some cargo crates : 
 * - anyhow
 * - ctrlc
 * - gst (package gstreamer, from gstreamer-rs.git)
 * - gst-audio (package gstreamer-audio, from gstreamer-rs.git)
 * - gst-video (package gstreamer-video, from gstreamer-rs.git)
 * - gstfallbackswitch (package gst-plugin-fallbackswitch, gst-plugins-rs.git)
 * - std::{fs,sync}
 * - xdg
 * May be run with : GST_TRACERS="pipeline-snapshot" GST_DEBUG_DUMP_DOT_DIR=. cargo run --example fd-to-twitch
 */

// https://github.com/matthew1000/gstreamer-cheat-sheet/blob/master/rtmp.md

use anyhow::{anyhow/*,bail*/,Context}; // You may use RUST_BACKTRACE=1 cargo run ... to get a backtrace
use gst::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const SOCKET_PATH: &str = "/tmp/lbs0";
const VIDEO_WIDTH: i32 = 1920; // px
const VIDEO_HEIGHT: i32 = 1080; // px
const VIDEO_FRAMERATE: i32 = 30;
const VIDEO_FRAMERATE_TWICE: i32 = 2*VIDEO_FRAMERATE;
const VIDEO_BITRATE: i32 = 4000; // kbit/s
const AUDIO_BITRATE: i32 = 160; // kbit/s
const AUDIO_CHANNELS: i32 = 2;
const AUDIO_SAMPLERATE: i32 = 48000; // Hz
const FALLBACK_SOURCE_URI: &str = "file:///home/ludolpif/demo.mkv";
const FALLBACK_TIMEOUT: f32 = 1.0; // seconds
const RTMP_CONFIG_FILE: &str = "rtmp_dest.txt";
const RTMP_EXTRA_STR: &str = "?bandwidthtest=true";
//const RTMP_EXTRA_STR: &str = ""; // REAL STREAM, USE WITH CAUTION

// For main() return type, see:
// - https://doc.rust-lang.org/std/process/trait.Termination.html
// - https://docs.rs/anyhow/latest/anyhow/
fn main() -> Result<(), anyhow::Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).context("Error setting Ctrl-C handler")?;

    let xdg_dirs = xdg::BaseDirectories::with_prefix("lbs")?;
    let rtmp_dest_path = xdg_dirs
        .place_config_file(RTMP_CONFIG_FILE)
        .context("cannot create configuration directory")?;
    let rtmp_dest_path_str = rtmp_dest_path.display().to_string();
    let rtmp_dest_read = std::fs::read_to_string(rtmp_dest_path)
        .context(format!("Cannot read {rtmp_dest_path_str}"))?;
    let rtmp_dest_str = rtmp_dest_read.trim(); // Strip plausible carriage return

    gst::init()?;
    gstfallbackswitch::plugin_register_static()?;

    //let audio_source_str = "audiotestsrc wave=8";
    let audio_source_str = "autoaudiosrc";
    let audio_caps_raw = gst_audio::AudioCapsBuilder::new()
        .format(gst_audio::AudioFormat::F32le)
        .rate(AUDIO_SAMPLERATE)
        .channels(AUDIO_CHANNELS)
        .layout(gst_audio::AudioLayout::Interleaved)
        .build();
    let audio_encoder_str = format!("avenc_aac bitrate={}", AUDIO_BITRATE*1000); // kbit/s -> bit/s

    let video_caps_raw = gst_video::VideoCapsBuilder::new()
        .format(gst_video::VideoFormat::Nv12)
        .width(VIDEO_WIDTH)
        .height(VIDEO_HEIGHT)
        .framerate(gst::Fraction::from_integer(VIDEO_FRAMERATE))
        .build();
    let video_encoder_str = format!("x264enc threads=0 bitrate={VIDEO_BITRATE} tune=zerolatency key-int-max={VIDEO_FRAMERATE_TWICE} cabac=1 bframes=2 ref=1");

    let video_caps_encoded = gst_video::VideoCapsBuilder::for_encoding("video/x-h264")
        .field("profile", "high")
        .build();

    println!("{audio_source_str} ! {audio_caps_raw} ! {audio_encoder_str}");
    println!("unixfdsrc ! fallbacksrc ! {video_caps_raw} ! {video_encoder_str} ! {video_caps_encoded}");

    let pipeline = create_pipeline_unixfd2rtmp(audio_source_str, audio_caps_raw, &audio_encoder_str.as_str(),
        SOCKET_PATH, video_caps_raw, video_encoder_str.as_str(), video_caps_encoded,
        gst::ClockTime::from_seconds_f32(FALLBACK_TIMEOUT), FALLBACK_SOURCE_URI,
        rtmp_dest_str, RTMP_EXTRA_STR)?;

    let bus = pipeline.bus().ok_or(anyhow!("pipeline without bus, may be invaid or incomplete (somesrc ! ... ! somesink)"))?;
    pipeline.set_state(gst::State::Playing).context("Failed to set_state Playing on pipeline")?;

    // GStreamer pipeline event loop
    for msg in bus.iter_timed(gst::ClockTime::SECOND) {
        match msg.view() {
            gst::MessageView::Eos(eos) => {
                println!("End Of Stream received {eos:?}");
                break;
            },
            gst::MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        };
        if ! running.load(Ordering::SeqCst) { break; }
    }

    // Gracefull stop 
    pipeline.debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pipeline-before-stop");
    pipeline.set_state(gst::State::Null).context("Failed to stop the pipeline")?;

    Ok(())
}

fn create_pipeline_unixfd2rtmp(
    audio_source_str: &str,
    audio_caps_raw: gst::caps::Caps,
    audio_encoder_str: &str,
    video_socket_path: &str,
    video_caps_raw: gst::caps::Caps,
    video_encoder_str: &str,
    video_caps_encoded: gst::caps::Caps,
    fallback_timeout: gst::ClockTime,
    fallback_uri: &str,
    rtmp_dest_str: &str,
    rtmp_extra_str: &str,
    ) -> Result<gst::Pipeline, anyhow::Error> {
    let pipeline = gst::Pipeline::with_name("unixfd2rtmp");
    let mainsrc = gst::ElementFactory::make("unixfdsrc")
        .property("socket-path", video_socket_path)
        .build()?;
    let fallbacksrc = gst::ElementFactory::make("fallbacksrc")
        //.property("enable-audio", false)
        .property("fallback-uri", fallback_uri)
        .property("restart-on-eos", true)
        .property("timeout", fallback_timeout)
        .property("source", mainsrc)
        .build()?;
    let fallback_audio_sink = gst::ElementFactory::make("autoaudiosink").build()?;
    let videoconvert = gst::ElementFactory::make("videoconvert").build()?;
    let video_capsfilter_raw = gst::ElementFactory::make("capsfilter")
        .property("caps", &video_caps_raw)
        .build()?;
    let video_encoder = parse_gst_element(video_encoder_str).context("Failed to parse video_encoder element")?;
    let video_capsfilter_encoded = gst::ElementFactory::make("capsfilter")
        .property("caps", &video_caps_encoded)
        .build()?;
    let audio_source = parse_gst_element(audio_source_str).context("Failed to parse audio_source element")?;
    let audioconvert = gst::ElementFactory::make("audioconvert").build()?;
    let audio_capsfilter_raw = gst::ElementFactory::make("capsfilter")
        .property("caps", &audio_caps_raw)
        .build()?;
    let audio_encoder = parse_gst_element(audio_encoder_str).context("Failed to parse audio_encoder element")?;
    let muxer = gst::ElementFactory::make("flvmux")
        .property("streamable", true)
        .build()?;
        /*
    let lastsink = gst::ElementFactory::make("rtmpsink")
        .property("location", format!("{rtmp_dest_str}{rtmp_extra_str} live=1"))
        .build()?;*/
    let lastsink_bin = parse_gst_bin_and_ghost_unlinked_pads("monitor_bin", "decodebin3 name=in ! videoconvert ! autovideosink in. ! audioconvert ! autoaudiosink")?;
    let lastsink_in_element = lastsink_bin.by_name("in").unwrap();
    let lastsink_in_sinkpad = lastsink_in_element.static_pad("sink").unwrap();
    let lastsink_bin_proxypad = gst::GhostPad::with_target(&lastsink_in_sinkpad)?;
    lastsink_bin.add_pad(&lastsink_bin_proxypad)?;
    let lastsink = lastsink_bin.upcast::<gst::Element>();

    pipeline.add_many([&fallbacksrc, &fallback_audio_sink])?;
    pipeline.add_many([&videoconvert, &video_capsfilter_raw, &video_encoder, &video_capsfilter_encoded])?;
    pipeline.add_many([&audio_source, &audioconvert, &audio_capsfilter_raw, &audio_encoder])?;
    pipeline.add_many([&muxer, &lastsink])?;
    // Not linking fallbacksrc here because it has "Sometimes" pads (not Always nor Request).
    // See tutorial 3 and 7 here: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/tree/main/tutorials/src/bin
    // On "Failed to link elements" error here, you may re-run with GST_DEBUG=4 cargo run ...
    gst::Element::link_many([&videoconvert, &video_capsfilter_raw, &video_encoder, &video_capsfilter_encoded, &muxer])?;
    gst::Element::link_many([&audio_source, &audioconvert, &audio_capsfilter_raw, &audio_encoder, &muxer])?;
    gst::Element::link_many([&muxer, &lastsink])?;

    // Register a pad-added signal handler to connect pads when they will be available
    // (when pipeline will swtich to state Playing)
    fallbacksrc.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());

        if src_pad.name() == "video" {
            let sink_pad = videoconvert
                .static_pad("sink")
                .expect("Failed to get static sink pad from videoconvert");
            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Link failed to video_encoder.");
            } else {
                println!("Link succeeded to video_encoder.");
            }
        } else if src_pad.name() == "audio" {
            let sink_pad = fallback_audio_sink
                .static_pad("sink")
                .expect("Failed to get static sink pad from fallback_audio_sink");
            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Link failed from fallbacksrc to fallback_audio_sink");
            } else {
                println!("Link succeeded from fallbacksrc to fallback_audio_sink");
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
                Err(anyhow::Error::new(err).context(format!("Missing gstreamer element: {:?}", context.missing_elements())))
            } else {
                Err(anyhow::Error::new(err).context(format!("Failed to parse element: {element_description}")))
            }
        }
    }
}

fn parse_gst_bin_and_ghost_unlinked_pads(bin_name: &str, bin_description: &str) -> Result<gst::Bin, anyhow::Error> {
    let mut context = gst::ParseContext::new();
    match gst::parse::bin_from_description_with_name_full(
        &bin_description,
        false, // TODO ghost_unlinked_pads
        &bin_name,
        Some(&mut context),
        gst::ParseFlags::empty(),
        ) {
        Ok(bin) => Ok(bin.downcast::<gst::Bin>().unwrap()),
        Err(err) => {
            if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                Err(anyhow::Error::new(err).context(format!("Missing gstreamer elements: {:?}", context.missing_elements())))
            } else {
                Err(anyhow::Error::new(err).context(format!("Failed to parse bin: {bin_description}")))
            }
        }
    }
}
