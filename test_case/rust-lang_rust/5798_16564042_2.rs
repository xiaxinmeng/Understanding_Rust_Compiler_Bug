
compile-fail/issue-2370.rs:18:11: 18:16 error: internal compiler error: 0'th deref is of a non-deref'able type `cat`
compile-fail/issue-2370.rs:18     error!(*nyan);
                                         ^~~~~
note: in expansion of fmt!
<core-macros>:6:24: 6:43 note: expansion site
<core-macros>:4:4: 11:5 note: in expansion of error!
compile-fail/issue-2370.rs:18:4: 18:18 note: expansion site
