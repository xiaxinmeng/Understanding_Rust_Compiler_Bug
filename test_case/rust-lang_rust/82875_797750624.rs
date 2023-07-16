rust
                } else if list_contains_name(&items[..], sym::always) {
                    if tcx.sess.opts.debugging_opts.instrument_coverage {
                        InlineAttr::Hint
                    } else {
                        InlineAttr::Always
                    }
