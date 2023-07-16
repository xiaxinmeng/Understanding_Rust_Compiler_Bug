
rprichard@ryan:~/mess$ rustc test.rs
test.rs:6:17: 6:32 error: unresolved name `fake`
test.rs:6     () => { 1 + nested_expr!(); } //~ ERROR unresolved name
                          ^~~~~~~~~~~~~~~
error: aborting due to previous error
