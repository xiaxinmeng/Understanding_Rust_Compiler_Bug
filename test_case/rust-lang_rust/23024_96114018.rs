
~/Downloads $ rustc test.rs
test.rs:11:35: 11:37 error: angle-bracket notation is not stable when used with the `Fn` family of traits, use parentheses [E0215]
test.rs:11     println!("{:?}",(vfnfer[0] as Fn)(3));
                                             ^~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:11:5: 11:43 note: expansion site
test.rs:11:35: 11:37 help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
test.rs:11:35: 11:37 error: wrong number of type arguments: expected 1, found 0 [E0243]
test.rs:11     println!("{:?}",(vfnfer[0] as Fn)(3));
                                             ^~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:11:5: 11:43 note: expansion site
test.rs:11:35: 11:37 error: the value of the associated type `Output` (from the trait `core::ops::FnOnce`) must be specified [E0191]
test.rs:11     println!("{:?}",(vfnfer[0] as Fn)(3));
                                             ^~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:11:5: 11:43 note: expansion site
test.rs:11:22: 11:37 error: illegal cast; cast from fat pointer: `Box<core::any::Any>` as `core::ops::Fn<[type error]>`
test.rs:11     println!("{:?}",(vfnfer[0] as Fn)(3));
                                ^~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:11:5: 11:43 note: expansion site
error: aborting due to 4 previous errors
~/Downloads $ rustc test.rs
test.rs:9:35: 9:37 error: angle-bracket notation is not stable when used with the `Fn` family of traits, use parentheses [E0215]
test.rs:9     println!("{:?}",(vfnfer[0] as Fn)(3));
                                            ^~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:9:5: 9:43 note: expansion site
test.rs:9:35: 9:37 help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
test.rs:9:35: 9:37 error: wrong number of type arguments: expected 1, found 0 [E0243]
test.rs:9     println!("{:?}",(vfnfer[0] as Fn)(3));
                                            ^~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:9:5: 9:43 note: expansion site
test.rs:9:35: 9:37 error: the value of the associated type `Output` (from the trait `core::ops::FnOnce`) must be specified [E0191]
test.rs:9     println!("{:?}",(vfnfer[0] as Fn)(3));
                                            ^~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:9:5: 9:43 note: expansion site
test.rs:9:22: 9:37 error: illegal cast; cast from fat pointer: `Box<core::any::Any>` as `core::ops::Fn<[type error]>`
test.rs:9     println!("{:?}",(vfnfer[0] as Fn)(3));
                               ^~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
test.rs:9:5: 9:43 note: expansion site
error: aborting due to 4 previous errors
