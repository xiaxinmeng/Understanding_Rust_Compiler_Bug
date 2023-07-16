rust
pub trait Layout {
    fn size_rules(&mut self, size_mgr: SizeMgr, axis: AxisInfo) -> SizeRules;
    fn set_rect(&mut self, mgr: &mut ConfigMgr, rect: Rect, align: AlignHints);
    fn draw(&mut self, draw: DrawMgr);
}
pub trait Widget: Layout + .. {
    fn configure(&mut self, mgr: &mut ConfigMgr);
    fn handle_event(&mut self, mgr: &mut EventMgr, event: Event) -> Response;
    fn handle_message(&mut self, mgr: &mut EventMgr, index: usize);
    // (and more)
}
