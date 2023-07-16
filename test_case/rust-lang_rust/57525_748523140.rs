rust
    /// Supplies a new frame to WebRender.
    ///
    /// Non-blocking, it notifies a worker process which processes the display list.
    ///
    /// Note: Scrolling doesn't require an own Frame.
    pub fn set_display_list(
        /// Target Document ID.
        &mut self,
        /// The unique Frame ID, monotonically increasing.
        epoch: Epoch,
        /// The background color of this pipeline.
        background: Option<ColorF>,
        /// The size of the viewport for this frame.
        viewport_size: LayoutSize,
        /// * `pipeline_id`: The ID of the pipeline that is supplying this display list.
        /// * `content_size`: The total screen space size of this display list's display items.
        /// * `display_list`: The root Display list used in this frame.
        (pipeline_id, content_size, display_list): (PipelineId, LayoutSize, BuiltDisplayList),
        /// If a previous frame exists which matches this pipeline
        /// id, this setting determines if frame state (such as scrolling
        /// position) should be preserved for this new display list.
        preserve_frame_state: bool,
    )
