
/home/alex/code/rust3/src/libcollections/string.rs:1022:18: 1022:28 error: multiple applicable methods in scope [E0034]
/home/alex/code/rust3/src/libcollections/string.rs:1022         (**self).as_slice()
                                                                         ^~~~~~~~~~
/home/alex/code/rust3/src/libcollections/string.rs:1022:18: 1022:28 note: candidate #1 is defined in an impl of the trait `core::slice::AsSlice` for the type `&'a _`
/home/alex/code/rust3/src/libcollections/string.rs:1022         (**self).as_slice()
                                                                         ^~~~~~~~~~
/home/alex/code/rust3/src/libcollections/string.rs:1022:18: 1022:28 note: candidate #2 is defined in an impl of the trait `core::slice::AsSlice` for the type `&'a mut _`
/home/alex/code/rust3/src/libcollections/string.rs:1022         (**self).as_slice()
                                                                         ^~~~~~~~~~
/home/alex/code/rust3/src/libcollections/string.rs:1022:18: 1022:28 note: candidate #3 is defined in an impl of the trait `core::str::Str` for the type `str`
/home/alex/code/rust3/src/libcollections/string.rs:1022         (**self).as_slice()
                                                                         ^~~~~~~~~~
/home/alex/code/rust3/src/libcollections/string.rs:1022:18: 1022:28 note: candidate #4 is defined in an impl of the trait `core::slice::AsSlice` for the type `str`
/home/alex/code/rust3/src/libcollections/string.rs:1022         (**self).as_slice()
                                                                         ^~~~~~~~~~
/home/alex/code/rust3/src/libcollections/string.rs:1022:18: 1022:28 note: candidate #5 is defined in an impl of the trait `core::str::Str` for the type `&'a _`
/home/alex/code/rust3/src/libcollections/string.rs:1022         (**self).as_slice()
                                                                         ^~~~~~~~~~
