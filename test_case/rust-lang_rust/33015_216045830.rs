 Rust

    pub fn emit(&mut self) {
        if self.cancelled() {
            return;
        }

        self.emitter.borrow_mut().emit_struct(&self);
        self.cancel();

        // if self.is_fatal() {
        //     panic!(FatalError);
        // }
    }
