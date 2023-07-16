 text
hello.rs:3:30: 3:41 error: type mismatch: the type `[closure@hello.rs:2:13: 2:18]` implements the trait `core::ops::FnMut<()>`, but the trait `for<'r> core::ops::FnMut<(&'r (),)>` is required (expected tuple, found ()) [E0281]
hello.rs:3     let g: Box<FnMut(&())> = Box::new(f) as Box<FnMut(&())>;
                                        ^~~~~~~~~~~
hello.rs:3:30: 3:41 help: run `rustc --explain E0281` to see a detailed explanation
hello.rs:3:30: 3:41 note: required for the cast to the object type `for<'r> core::ops::FnMut(&'r ())`
hello.rs:3:30: 3:41 error: type mismatch: the type `[closure@hello.rs:2:13: 2:18]` implements the trait `core::ops::FnOnce<()>`, but the trait `for<'r> core::ops::FnOnce<(&'r (),)>` is required (expected tuple, found ()) [E0281]
hello.rs:3     let g: Box<FnMut(&())> = Box::new(f) as Box<FnMut(&())>;
                                        ^~~~~~~~~~~
hello.rs:3:30: 3:41 help: run `rustc --explain E0281` to see a detailed explanation
hello.rs:3:30: 3:41 note: required for the cast to the object type `for<'r> core::ops::FnMut(&'r ())`
error: aborting due to 2 previous errors
