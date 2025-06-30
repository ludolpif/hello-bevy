use bevy::prelude::{Bundle,Component,Vec2};
//use std::path::Path;

/* Profile */

#[derive(Component)]
pub struct VideoSink;

#[derive(Component)]
pub struct RenderResolution(Vec2);

#[derive(Component)]
pub struct OutputResolution(Vec2);

#[derive(Component)]
pub struct OutputFramerate(f32);

#[derive(Component)]
pub struct OutputAudioSampleRate(i32);

#[derive(Component)]
pub enum OutputDestination {
    Stream,
    LocalRecord,
    StreamAndLocalRecord
}

#[derive(Component)]
pub enum AudioEncoder {
    AAC,
}

#[derive(Component)]
pub struct AudioEncoderBitrate(i32);

#[derive(Component)]
pub enum VideoEncoder {
    X264,
    NvEncH264,
}

#[derive(Component)]
pub enum VideoEncoderProfile {
    High,
}

#[derive(Component)]
pub struct VideoEncoderBitrate(i32);

#[derive(Component)]
pub struct VideoEncoderKeyFrameEvery(i32);


#[derive(Bundle)]
struct VideoSinkBundle {
    tag: VideoSink,
    rr: RenderResolution,
    or: OutputResolution,
    ar: OutputFramerate,
    vr: OutputAudioSampleRate,
    dest: OutputDestination,
    ae: AudioEncoder,
    ab: AudioEncoderBitrate,
    ve: VideoEncoder,
    vb: VideoEncoderBitrate,
}

/* Stream destination */

#[derive(Component)]
pub enum StreamProtocol {
    RTMP,
}

#[derive(Component)]
pub enum StreamService {
    Twitch,
}

#[derive(Component)]
pub struct StreamServer(String);

//FIXME how to have some secrets management ?
#[derive(Component)]
pub struct StreamKeyId(i32);

#[derive(Component)]
pub enum StreamMode {
    Diffusion,
    BandwidthTesting,
}

/* LocalRecord destination */

#[derive(Component)]
pub struct RecordFolder(String);

#[derive(Component)]
pub enum RecordContainer {
    Matroska,
}

#[derive(Component)]
pub struct FileNameFmt(String);

