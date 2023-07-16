
/home/mneumann/rust/src/liblibc/lib.rs:2943:47: 2943:53 error: unresolved import
 `types::os::arch::c95::c_uint`. Could not find `arch` in `types::os`.
/home/mneumann/rust/src/liblibc/lib.rs:2943             use types::os::arch::c95
::{c_int, c_uint};

          ^~~~~~
/home/mneumann/rust/src/liblibc/lib.rs:2965:17: 2965:44 error: unresolved import
 `types::os::arch::c95::c_int`. Could not find `arch` in `types::os`.
/home/mneumann/rust/src/liblibc/lib.rs:2965             use types::os::arch::c95
::c_int;
                                                            ^~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/mneumann/rust/src/liblibc/lib.rs:2966:17: 2966:49 error: unresolved import
 `types::os::arch::posix88::mode_t`. Could not find `arch` in `types::os`.
/home/mneumann/rust/src/liblibc/lib.rs:2966             use types::os::arch::pos
ix88::mode_t;
                                                            ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/mneumann/rust/src/liblibc/lib.rs:3209:17: 3209:44 error: unresolved import
 `types::os::arch::c95::c_int`. Could not find `arch` in `types::os`.
/home/mneumann/rust/src/liblibc/lib.rs:3209             use types::os::arch::c95
::c_int;
                                                            ^~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/mneumann/rust/src/liblibc/lib.rs:150:54: 150:58 error: unresolved import (
maybe you meant `stat::*`?)
/home/mneumann/rust/src/liblibc/lib.rs:150 pub use funcs::posix88::stat_::{chmod
, fstat, mkdir, stat};

                ^~~~
error: aborting due to 115 previous errors
gmake: *** [x86_64-unknown-dragonfly/stage1/lib/rustlib/x86_64-unknown-dragonfly
/lib/stamp.libc] Error 101
