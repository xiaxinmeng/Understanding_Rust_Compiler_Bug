
rprichard@ryan:~/mess$ ~/work/rust-staging2/build/install/bin/rustc test.rs
test.rs:6:17: 6:32 error: unresolved name `fake`
test.rs:6     () => { 1 + nested_expr!(); } //~ ERROR unresolved name
                          ^~~~~~~~~~~~~~~
test.rs:5:1: 7:2 note: in expansion of call_nested_expr_sum!
test.rs:10:5: 10:29 note: expansion site
error: aborting due to previous error
