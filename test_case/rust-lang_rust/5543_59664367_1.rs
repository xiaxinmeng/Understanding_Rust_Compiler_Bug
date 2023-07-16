
<anon>:8:18: 8:34 error: the trait `std::io::Reader` is not implemented for the type `std::io::Reader`
<anon>:8     let _m = bar(r as Box<Reader>);
                          ^~~~~~~~~~~~~~~~
<anon>:8:18: 8:34 note: the trait `std::io::Reader` must be implemented for the cast to the object type `std::io::Reader`
<anon>:8     let _m = bar(r as Box<Reader>);
                          ^~~~~~~~~~~~~~~~
error: aborting due to previous error
