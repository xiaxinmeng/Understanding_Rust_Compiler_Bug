
/home/brian/dev/rust3/src/test/compile-fail/multiple-methods-in-scope.rs:26:4: 26:15 error: multiple applicable methods in scope
/home/brian/dev/rust3/src/test/compile-fail/multiple-methods-in-scope.rs:26     i.foo_it();
                                                                                ^~~~~~~~~~~
/home/brian/dev/rust3/src/test/compile-fail/multiple-methods-in-scope.rs:21:19: 21:39 note: candidate #1 is `__extensions__::foo_it`
/home/brian/dev/rust3/src/test/compile-fail/multiple-methods-in-scope.rs:21 impl Foo for int { fn foo_it(&self) { } }
                                                                                               ^~~~~~~~~~~~~~~~~~~~
/home/brian/dev/rust3/src/test/compile-fail/multiple-methods-in-scope.rs:22:19: 22:39 note: candidate #2 is `__extensions__::foo_it`
/home/brian/dev/rust3/src/test/compile-fail/multiple-methods-in-scope.rs:22 impl Bar for int { fn foo_it(&self) { } }
                                                                                               ^~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
