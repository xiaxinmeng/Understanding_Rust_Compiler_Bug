
let lo = self.span;
let mut too_many = false;
while self.ch_is('#') {
    too_many = true;
    self.bump();
}
if too_many {
    let sp = lo.to(self.span);
    let mut err = self.sess.span_diagnostic.struct_span_err(sp, "too many `#` when terminating raw string");
    err.span_label(sp, "too many `#` for raw string");
    err.hidden_span_suggestion(
        sp,
        "remove the unneeded `#`"
        String::new(),
        Applicability::MachineApplicable,
    );
    err.emit();
}
