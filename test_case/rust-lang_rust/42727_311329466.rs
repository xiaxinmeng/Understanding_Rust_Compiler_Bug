
[00:04:43] --- stderr
[00:04:43] error[E0432]: unresolved import `new::System`
[00:04:43]    --> src\liballoc_system\lib.rs:282:9
[00:04:43]     |
[00:04:43] 282 |     use new::System;
[00:04:43]     |         ^^^^^^^^^^^ Maybe a missing `extern crate new;`?
[00:04:43]
[00:04:43] error[E0432]: unresolved import `new::alloc::heap::Alloc`
[00:04:43]    --> src\liballoc_system\lib.rs:283:28
[00:04:43]     |
[00:04:43] 283 |     use new::alloc::heap::{Alloc, AllocErr, Layout, CannotReallocInPlace};
[00:04:43]     |                            ^^^^^ Maybe a missing `extern crate new;`?
[00:04:43]
[00:04:43] error[E0432]: unresolved import `new::alloc::heap::AllocErr`
[00:04:43]    --> src\liballoc_system\lib.rs:283:35
[00:04:43]     |
[00:04:43] 283 |     use new::alloc::heap::{Alloc, AllocErr, Layout, CannotReallocInPlace};
[00:04:43]     |                                   ^^^^^^^^ Maybe a missing `extern crate new;`?
[00:04:43]
[00:04:43] error[E0432]: unresolved import `new::alloc::heap::Layout`
[00:04:43]    --> src\liballoc_system\lib.rs:283:45
[00:04:43]     |
[00:04:43] 283 |     use new::alloc::heap::{Alloc, AllocErr, Layout, CannotReallocInPlace};
[00:04:43]     |                                             ^^^^^^ Maybe a missing `extern crate new;`?
[00:04:43]
[00:04:43] error[E0432]: unresolved import `new::alloc::heap::CannotReallocInPlace`
[00:04:43]    --> src\liballoc_system\lib.rs:283:53
[00:04:43]     |
[00:04:43] 283 |     use new::alloc::heap::{Alloc, AllocErr, Layout, CannotReallocInPlace};
[00:04:43]     |                                                     ^^^^^^^^^^^^^^^^^^^^ Maybe a missing `extern crate new;`?
[00:04:43]
[00:04:43] error: cannot continue compilation due to previous error
