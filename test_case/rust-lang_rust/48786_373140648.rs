rust
pub fn must_not_eliminate_frame_pointers(&self) -> bool {
        if let Some(x) = self.opts.cg.force_frame_pointers {
            x
        } else {
            !self.target.target.options.eliminate_frame_pointer ||
            self.opts.debuginfo != DebugInfoLevel::NoDebugInfo
        }
}
