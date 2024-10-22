/* Needs some cargo crates :
 * - anyhow
 * - ctrlc
 * - gst (package gstreamer, from gstreamer-rs.git)
 * - gst-audio (package gstreamer-audio, from gstreamer-rs.git)
 * - gst-video (package gstreamer-video, from gstreamer-rs.git)
 * - std::{fs,sync}
 * May be run with : GST_DEBUG_DUMP_DOT_DIR=. cargo run --example playsink
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
    let pipeline_name = "playsink-demo";

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Ctrl-C set running to false");
    })
    .context("Error setting Ctrl-C handler")?;

    gst::init()?;

    let pipeline = create_pipeline(pipeline_name)?;


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

fn create_pipeline(name: &str) -> Result<gst::Pipeline, anyhow::Error> {
    let pipeline = gst::Pipeline::with_name(&name);

    let filesrc = gst::ElementFactory::make("filesrc")
        .property("location", "/home/ludolpif/demo.mkv")
        .build()?;
    let decodebin = gst::ElementFactory::make("decodebin3")
        .build()?;
    let playsink = gst::ElementFactory::make("playsink")
        .build()?;

    pipeline.add_many([&filesrc, &decodebin, &playsink])?;
    gst::Element::link_many([&filesrc, &decodebin])?;

    // Register a pad-added signal handler to connect pads when they will be available
    // (when pipeline will switch to state Playing)
    decodebin.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());
        src.downcast_ref::<gst::Bin>()
            .unwrap()
            .debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pad-added");
        
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
