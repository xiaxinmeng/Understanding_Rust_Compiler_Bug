
compile-fail/issue-2370.rs:18:11: 18:16 error: can only dereference structs with one anonymous field
compile-fail/issue-2370.rs:18:11: 18:16 error: internal compiler error: 0'th deref is of a non-deref'able type `cat`
compile-fail/issue-2370.rs:18     error!(*nyan);
                                                                                    ^~~~~
note: in expansion of fmt!
<core-macros>:6:24: 6:43 note: expansion site
<core-macros>:4:4: 11:5 note: in expansion of error!
/home/ymin/share/rust/rustmozilla/src/test/compile-fail/issue-2370.rs:18:4: 18:18 note: expansion site
error: aborting due to 2 previous errors
