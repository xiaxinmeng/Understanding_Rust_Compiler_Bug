
methodnameclash.rs:7:28: 7:35 error: mismatched types: expected `int` but found `bool` (int vs bool)
methodnameclash.rs:7 fn ab<T:A B>(x: T) -> int { x.foo() }
                                                 ^~~~~~~
methodnameclash.rs:8:29: 8:36 error: mismatched types: expected `bool` but found `int` (bool vs int)
methodnameclash.rs:8 fn ba<T:B A>(x: T) -> bool { x.foo() }
                                                  ^~~~~~~
error: aborting due to previous errors
