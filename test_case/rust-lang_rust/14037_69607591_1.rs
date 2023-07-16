
foo.rs:3:30: 3:60 error: the trait `for<'r> core::ops::Fn(&'r ())` is not implemented for the type `closure[foo.rs:2:13: 2:21]`
foo.rs:3     let g: Box<FnMut(&())> = Box::new(f) as Box<FnMut(&())>;
                                      ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

