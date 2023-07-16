 rust
    pub fn emit(&self, msp: &MultiSpan, msg: &str, lvl: Level) {
        if lvl == Warning && !self.can_emit_warnings {
            return;
        }
        let mut db = DiagnosticBuilder::new(self, lvl, msg);
        db.set_span(msp.clone());
        db.emit();
        if !self.continue_after_error.get() { // (** HERE **)
            self.abort_if_errors();
        }
    }
