rust
                    self.report_method_error(method_name.span,
                                             expr_t,
                                             method_name.node,
                                             Some(rcvr),
                                             error,
                                             Some(args));
