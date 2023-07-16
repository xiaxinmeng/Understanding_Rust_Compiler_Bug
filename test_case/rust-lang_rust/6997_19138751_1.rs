
foo.rs:17:24: 17:35 error: cannot borrow `(*foo)[]` as mutable more than once at a time
foo.rs:17             return Some(&mut foo[idx])
                                  ^~~~~~~~~~~
foo.rs:17:24: 17:35 note: second borrow of `(*foo)[]` as mutable occurs here
foo.rs:17             return Some(&mut foo[idx])
                                  ^~~~~~~~~~~
error: aborting due to previous error
