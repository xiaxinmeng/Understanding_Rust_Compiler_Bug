 Rust

                #[inline(never)]
                fn do_metaloadfn(loadfn: &mut FnMut(&str) -> *const __gl_imports::raw::c_void,
                                 symbol: &str,
                                 symbols: &[&str])
                                 -> *const __gl_imports::raw::c_void
                {
                    let mut ptr = loadfn(symbol);
                    if ptr.is_null() {
                        for &sym in symbols {
                            ptr = loadfn(sym);
                            if !ptr.is_null() { break; }
                        }
                    }
                    ptr
                }
                let mut metaloadfn = |symbol: &str, symbols: &[&str]| {
                    do_metaloadfn(&mut loadfn, symbol, symbols)
                };
