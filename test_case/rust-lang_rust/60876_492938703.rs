
';' => { 
    self.bump(); 
    if self.ch_is(';') {
        self.bump();
        let mut err = self.sess.span_diagnostic.struct_span_warn(self.sp, "redundant `;` detected");
        ...
        err.emit();
    }
    Ok(token::Semi) 
} 
