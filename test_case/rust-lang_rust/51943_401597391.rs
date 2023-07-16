
[01:08:34] error[E0432]: unresolved import `rustc::middle::const_val`
[01:08:34]   --> tools/miri/src/lib.rs:27:5
[01:08:34]    |
[01:08:34] 27 | use rustc::middle::const_val;
[01:08:34]    |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `const_val` in `middle`
[01:08:34] 
[01:08:34] error[E0432]: unresolved import `rustc::middle::const_val`
[01:08:34]   --> tools/miri/src/validation.rs:11:20
[01:08:34]    |
[01:08:34] 11 | use rustc::middle::const_val::ConstVal;
[01:08:34]    |                    ^^^^^^^^^ Could not find `const_val` in `middle`
[01:08:34] 
[01:08:36] error: aborting due to 2 previous errors
