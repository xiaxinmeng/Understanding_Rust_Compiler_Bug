rust
match f(self) {
                            Ok(t) => {
                                v.push(t);
                                continue;
                            },
                            Err(mut e) => {
                                //If f errored, it should be because we're missing a bracket (and thus shouldn't have parsed f).
                                e.cancel();
                                self.expect_one_of(&[], unref_kets)?;
                                //The double cloned() is ugly but is necessary due to kets being a slice of references.
                                break;
                            }
                        }
