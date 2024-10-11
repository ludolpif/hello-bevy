use anyhow::Error;
use derive_more::{Display as DmDisplay, Error as DmError};
use gst::prelude::*;
use gst_video::prelude::*;

#[derive(Debug, DmDisplay, DmError)]
#[display("Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const FRAMERATE: gst::Fraction = gst::Fraction::new_raw(60,1);

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let element_str = "appsrc name=source ! unixfdsink socket-path=/tmp/lbs0";
//    let element_str = "videotestsrc name=source ! unixfdsink socket-path=/tmp/lbs0";

    let mut context = gst::ParseContext::new();
    let element = match gst::parse::launch_full(
        &element_str,
        Some(&mut context),
        gst::ParseFlags::empty(),
    ) {
        Ok(element) => element,
        Err(err) => {
            // XXX May error_msg!() could help ? Or usefull only inside plugins ?
            // https://gstreamer.pages.freedesktop.org/gstreamer-rs/stable/latest/docs/src/gstreamer/error.rs.html#7
            if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                return Err(ErrorMessage {
                    src: "gst::parse::launch_full".into(),
                    error: err,
                    debug: Some(glib::GString::format(format_args!("Missing element(s): {:?}", context.missing_elements()))),
                }.into());
            } else {
                return Err(ErrorMessage {
                    src: "gst::parse::launch_full".into(),
                    error: err,
                    debug: Some("Failed to parse pipeline".into()),
                }.into());
            }
        }
    };
    // Casting Between Element Types in Rust
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/issues/487
    // GstPipeline pipeline = GST_PIPELINE(element); // C-style equivalent
    let pipeline = element.downcast::<gst::Pipeline>().unwrap();

    let srcname = "source";
    let src = match pipeline.by_name(srcname) {
        Some(src) => src,
        None => {
            return Err(Error::msg(format!("Can't find {srcname} in this pipeline: {element_str}")));
        }
    };
    let appsrc = match src.downcast::<gst_app::AppSrc>() {
        Ok(appsrc) => appsrc,
        Err(_err) => {
            //return Err(Error::msg(format!("{srcname} in this pipeline is not a appsrc: {element_str}")));
            println!("{srcname} in this pipeline is not an appsrc: {element_str}, going without any callbacks nor video_caps !");
            return Ok(pipeline)
        }
    };

    // Specify the format we want to provide as application into the pipeline
    // by creating a video info with the given format and creating caps from it for the appsrc element.
    let video_info =
        gst_video::VideoInfo::builder(gst_video::VideoFormat::DmaDrm, WIDTH, HEIGHT)
            .fps(FRAMERATE)
            .build()
            .expect("Failed to create video info");
    let video_info = gst_video::VideoInfoDmaDrm::new(video_info, gst_video::VideoFormat::Nv12.to_fourcc(), 0x0100000000000001);
    let orig_caps = appsrc.caps();
    println!("orig video_caps are {orig_caps:?}");
    let video_caps = video_info.to_caps().unwrap();
    println!("wanted video_caps are {video_caps:?}");

    appsrc.set_caps(Some(&video_caps));
    appsrc.set_format(gst::Format::Time);
    /*
    // Our frame counter, that is stored in the mutable environment
    // of the closure of the need-data callback
    //
    // Alternatively we could also simply start a new thread that
    // pushes a buffer to the appsrc whenever it wants to, but this
    // is not really needed here. It is *not required* to use the
    // need-data callback.
    let mut i = 0;
    appsrc.set_callbacks(
        // Since our appsrc element operates in pull mode (it asks us to provide data),
        // we add a handler for the need-data callback and provide new data from there.
        // In our case, we told gstreamer that we do 2 frames per second. While the
        // buffers of all elements of the pipeline are still empty, this will be called
        // a couple of times until all of them are filled. After this initial period,
        // this handler will be called (on average) twice per second.
        gst_app::AppSrcCallbacks::builder()
            .need_data(move |appsrc, _| {
                // We only produce 100 frames
                if i == 100 {
                    let _ = appsrc.end_of_stream();
                    return;
                }

                println!("Producing frame {i}");

                let r = if i % 2 == 0 { 0 } else { 255 };
                let g = if i % 3 == 0 { 0 } else { 255 };
                let b = if i % 5 == 0 { 0 } else { 255 };

                //FIXME unixfd want "buffers with FD memories"
                //https://discourse.gstreamer.org/t/dmabuf-wrapped-to-gstbuffer-for-appsrc/1538
                //https://gstreamer.freedesktop.org/documentation/allocators/gstshmallocator.html?gi-language=c
                //https://gstreamer.freedesktop.org/documentation/gstreamer/gstbufferpool.html?gi-language=c
                
                // Create the buffer that can hold exactly one BGRx frame.
                let mut buffer = gst::Buffer::with_size(video_info.size()).unwrap();
                {
                    let buffer = buffer.get_mut().unwrap();
                    // For each frame we produce, we set the timestamp when it should be displayed
                    // (pts = presentation time stamp)
                    // The autovideosink will use this information to display the frame at the right time.
                    buffer.set_pts(i * 500 * gst::ClockTime::MSECOND);

                    // At this point, buffer is only a reference to an existing memory region somewhere.
                    // When we want to access its content, we have to map it while requesting the required
                    // mode of access (read, read/write).
                    // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
                    let mut vframe =
                        gst_video::VideoFrameRef::from_buffer_ref_writable(buffer, &video_info)
                            .unwrap();

                    // Remember some values from the frame for later usage
                    let width = vframe.width() as usize;
                    let height = vframe.height() as usize;

                    // Each line of the first plane has this many bytes
                    let stride = vframe.plane_stride()[0] as usize;

                    // Iterate over each of the height many lines of length stride
                    for line in vframe
                        .plane_data_mut(0)
                        .unwrap()
                        .chunks_exact_mut(stride)
                        .take(height)
                    {
                        // Iterate over each pixel of 4 bytes in that line
                        for pixel in line[..(4 * width)].chunks_exact_mut(4) {
                            pixel[0] = b;
                            pixel[1] = g;
                            pixel[2] = r;
                            pixel[3] = 0;
                        }
                    }
                }

                i += 1;

                // appsrc already handles the error here
                let _ = appsrc.push_buffer(buffer);
            })
            .build(),
    );
 */
    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| glib::GString::from("UNKNOWN")),
                    error: err.error(),
                    debug: err.debug(),
                }
                .into());
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}
