
                        self.diagnostic()
                            .struct_span_err(span, reason)
                            .span_label(prev_attr_sp, prev_attr_msg)
                            .note("inner attributes, like `#![no_std]`, annotate the item \
                                   enclosing them, and are usually found at the beginning of \
                                   source files. Outer attributes, like `#[test]`, annotate the \
                                   item following them.")
                            .emit()
