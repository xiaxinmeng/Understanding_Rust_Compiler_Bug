 diff
     pub fn span_err<S: Into<MultiSpan>>(&self, sp: S, msg: &str) {
         self.emit(&sp.into(), msg, Error);
-        self.bump_err_count();
         self.panic_if_treat_err_as_bug();
     }
