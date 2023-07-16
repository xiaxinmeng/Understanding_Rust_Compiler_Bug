
foo.rs:7:18: 7:30 error: can only cast an ~-pointer to a ~-object, not a trait std::io::Reader
foo.rs:7     let _m = bar(r as ~Reader);
                          ^~~~~~~~~~~~
error: aborting due to previous error
