
hello.rs:2:25: 2:26 error: bare raw pointers are no longer allowed, you should likely use `*mut T`, but otherwise `*T` is now known as `*const T`
hello.rs:2       static StackBase: *u32;
                                   ^
hello.rs:7:46: 7:47 error: expected `:`, found `[`
hello.rs:7 static ISRVectors: &'static [u32] = &'static [
                                                        ^
