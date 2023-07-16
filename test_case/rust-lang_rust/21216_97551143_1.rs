
$ rustc main.rs
main.rs:8:13: 8:29 error: mismatched types:
 expected `&mut Box<core::ops::FnMut()>`,
    found `&mut Box<[closure main.rs:7:34: 7:39]>`
(expected trait core::ops::FnMut,
    found closure) [E0308]
main.rs:8   attach_fn(&mut optional_fn);
                      ^~~~~~~~~~~~~~~~
error: aborting due to previous error
