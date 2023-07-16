
/home/lkuper/rust/src/test/compile-fail/issue-2151.rs:3:20: 3:21 error: the type of this value must be known in this context
/home/lkuper/rust/src/test/compile-fail/issue-2151.rs:3         log (debug, i * 2);
                                                                            ^
/home/lkuper/rust/src/test/compile-fail/issue-2151.rs:3:20: 3:25 error: binary operation * cannot be applied to type `<V1>`
/home/lkuper/rust/src/test/compile-fail/issue-2151.rs:3         log (debug, i * 2);
                                                                            ^~~~~
