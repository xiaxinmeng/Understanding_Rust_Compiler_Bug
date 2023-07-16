 rust
...
                match const_eval::compare_lit_exprs(self.tcx, start, end) {
                    Some(Ordering::Less) |
                    Some(Ordering::Equal) => {}
                    Some(Ordering::Greater) => {
                        span_err!(self.tcx.sess, start.span, E0030,
                            "lower range bound must be less than or equal to upper");
                    }
                    None => {
                        self.tcx.sess.span_bug(
                            start.span, "literals of different types in range pat");
                    }
                }
...
