
bar.rs:14:18: 14:27 error: mismatched types: expected `extern "Rust" fn(&str)` but found `&fn(<V0>)` (expected extern fn but found fn)
bar.rs:14     do_spawn_stuff(|_s| { });
                            ^~~~~~~~~
bar.rs:14:18: 14:27 error: Unconstrained region variable #0
bar.rs:14     do_spawn_stuff(|_s| { });
                            ^~~~~~~~~
