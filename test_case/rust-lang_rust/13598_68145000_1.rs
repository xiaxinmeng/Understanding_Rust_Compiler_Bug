
phantom /tmp $ rustc --cfg variant1 tests.rs
tests.rs:10:9: 10:24 error: unresolved import `self::b::C`. Could not find `b` in `variant1`
tests.rs:10     use self::b::C as D;
                    ^~~~~~~~~~~~~~~
error: aborting due to previous error
phantom /tmp $ rustc --cfg variant2 tests.rs 
