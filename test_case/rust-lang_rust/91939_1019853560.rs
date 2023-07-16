rust
                if self.expr_ty.is_numeric() {
                    err.span_help(self.span, if self.expr_ty == fcx.tcx.types.i8 {
                        "try casting from `u8` instead"
                    } else if self.expr_ty == fcx.tcx.types.u32 {
                        "try `char::from_u32` instead"
                    } else {
                        "try `char::from_u32` instead (via a `u32`)"
                    });
                }
