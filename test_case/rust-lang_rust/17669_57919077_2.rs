
into_iter.rs:28:9: 28:10 error: unable to infer enough type information to locate the impl of the trait `core::kinds::Sized` for the type `<generic #5>`; type annotations required
into_iter.rs:28     let i = into_iter(v);  // ~ Err2
                        ^
into_iter.rs:28:9: 28:10 note: all local variables must have a statically known size
into_iter.rs:28     let i = into_iter(v);  // ~ Err2
                        ^
error: aborting due to previous error
