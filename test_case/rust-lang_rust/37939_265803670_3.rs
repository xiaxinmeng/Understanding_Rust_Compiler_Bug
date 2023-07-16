rust
                    // Carrier::translate(<expr>)
                    let discr = {
                        // expand <expr>
                        P(self.lower_expr(sub_expr))
                    };
