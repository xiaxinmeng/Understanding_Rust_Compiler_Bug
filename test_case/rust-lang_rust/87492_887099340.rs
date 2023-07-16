rs
unsafe fn unsf() {}
fn bad<T>() -> Box<dyn Iterator<Item = [(); { unsafe { || { unsf() } }; 4 }]>> { todo!() }
//                                                          ^^^^^^ shouldn't error, but does with this PR
