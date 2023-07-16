
x.rs:3:15: 3:23 error: no method named `into_raw` found for type `Box<_>` in the current scope
x.rs:3     let y = x.into_raw();
                     ^~~~~~~~
x.rs:3:15: 3:23 note: found the following associated functions; to be used as methods, functions must have a `self` parameter
x.rs:3:15: 3:23 note: candidate #1 is defined in an impl for the type `Box<_>`
x.rs:3     let y = x.into_raw();
                     ^~~~~~~~
