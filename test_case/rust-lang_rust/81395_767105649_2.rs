rust
#[derive(Debug)]
pub struct VideoFrameRef<T> {
    frame: ffi::GstVideoFrame,
    buffer: Option<T>,
    info: crate::VideoInfo,
    unmap: bool,
}
