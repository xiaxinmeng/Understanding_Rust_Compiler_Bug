
test-trait-visibility.rs:17:4: 17:7 error: multiple applicable methods in scope
test-trait-visibility.rs:17     s.x();
                                ^~~
test-trait-visibility.rs:7:17: 7:25 note: candidate #1 is `a::__extensions__::x`
test-trait-visibility.rs:7     impl S : A { fn x(){} }
                                            ^~~~~~~~
test-trait-visibility.rs:12:17: 12:25 note: candidate #2 is `b::__extensions__::x`
test-trait-visibility.rs:12     impl S : B { fn x(){} }
                                             ^~~~~~~~
error: aborting due to previous error
