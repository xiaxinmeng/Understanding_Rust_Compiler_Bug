rust
    fn build_dylib(&mut self, out_filename: &Path) {
        // On mac we need to tell the linker to let this library be rpathed
        if self.sess.target.is_like_osx {
            self.cmd.arg("-dynamiclib");
