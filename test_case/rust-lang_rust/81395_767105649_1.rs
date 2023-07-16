rust
impl<'a> ops::Deref for VideoFrameRef<&'a mut gst::BufferRef> {
    type Target = VideoFrameRef<&'a gst::BufferRef>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self as *const VideoFrameRef<&'a mut gst::BufferRef>
                as *const VideoFrameRef<&'a gst::BufferRef>)
        }
    }
}
