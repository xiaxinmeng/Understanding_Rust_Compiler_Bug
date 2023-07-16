
            ty::TyInt(ast::TyIs) => {
                FfiUnsafe("found Rust type `isize` in foreign module, while \
                          `libc::c_int` or `libc::c_long` should be used")
            }
            ty::TyUint(ast::TyUs) => {
                FfiUnsafe("found Rust type `usize` in foreign module, while \
                          `libc::c_uint` or `libc::c_ulong` should be used")
            }
