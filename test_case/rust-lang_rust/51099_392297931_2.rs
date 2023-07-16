rust
let before_span = self.span.clone();
            let maybe_err = f(self);
            match maybe_err {
                Ok(t) => {
                    v.push(t);
                }
                Err(e) => {
                    if before_span == self.span {
                        //If no input was consumed, assume we wanted a sequence ender.
                        self.expect_one_of(&[], unref_kets)?;
                        //The double cloned() is ugly but is necessary due to kets being a slice of references.
                        //If there was in fact a sequence ender, return the error the function gave us.
                    }
                    else {
                        debug!("parse_seq_to_before_tokens: No equality. Propogating error.");
                    }
                    return Err(e.into());
                }
            }
