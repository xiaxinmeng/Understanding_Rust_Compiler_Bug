
self.struct_span_err(self.prev_span, message)
    .span_label(self.prev_span, "this doc comment doesn't document anything")
    .emit();
