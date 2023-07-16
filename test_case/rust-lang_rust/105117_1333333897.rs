plain
    Finished release [optimized] target(s) in 2m 56s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (exit status: 254)
rustc 1.67.0-nightly (121cee6eb 2022-12-01)
binary: rustc
commit-hash: 121cee6eb144a14bd1abf5edb1c878e2533aa9b8
commit-date: 2022-12-01
commit-date: 2022-12-01
host: x86_64-unknown-linux-gnu
release: 1.67.0-nightly
LLVM version: 13.0.1

--- stderr
thread '<unnamed>' panicked at 'assertion failed: this.is_unique()', /checkout/library/alloc/src/sync.rs:1644:9
stack backtrace:
   0:     0x7f016c95fe32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   1:     0x7f016c9cf078 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7f016c9508c1 - std::io::Write::write_fmt::h115d78592252bf9d
   3:     0x7f016c95fbf5 - std::sys_common::backtrace::print::hd9ebfd55eb3388eb
   4:     0x7f016c963017 - std::panicking::default_hook::{{closure}}::h98850a6457a8782d
   5:     0x7f016c962cdb - std::panicking::default_hook::hef750a0d73d5e8e9
   6:     0x7f016d34a894 - rustc_driver[6405d95162ec2db2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f016c963923 - std::panicking::rust_panic_with_hook::ha62dd9a4e901383e
Build completed unsuccessfully in 0:03:47
   8:     0x7f016c963611 - std::panicking::begin_panic_handler::{{closure}}::h30577589484d4d4f
   9:     0x7f016c9603dc - std::sys_common::backtrace::__rust_end_short_backtrace::h8641e565b807163f
  10:     0x7f016c963322 - rust_begin_unwind
  11:     0x7f016c914033 - core::panicking::panic_fmt::ha29375e42f9e82fb
  12:     0x7f016c91410d - core::panicking::panic::hb659ada2b02be8b4
  13:     0x7f016c927a16 - alloc::sync::Arc<T>::drop_slow::h63351216a29d5be0
  14:     0x7f016c975393 - std::sys_common::thread_info::THREAD_INFO::__getit::destroy::h4eddde57fd78359c
  15:     0x7f016c6b5d9f - __call_tls_dtors
  16:     0x7f016c6b55c9 - <unknown>
  17:     0x7f016c6b5610 - exit
  18:     0x7f016c96c857 - std::sys::unix::os::exit::h34004398f9017db1
  19:     0x7f016c95e9af - std::process::exit::h65ff648367d53ca3
  20:     0x7f016d34c097 - rustc_driver[6405d95162ec2db2]::main
  21:     0x56395eabb177 - rustc_main[2dd750e4836bcfb6]::main
  22:     0x56395eabb133 - std[65b05b99172e1c7e]::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  23:     0x56395eabb159 - std[65b05b99172e1c7e]::rt::lang_start::<()>::{closure#0}
  24:     0x7f016c93ea24 - std::rt::lang_start_internal::hafd522f3051e9319
  25:     0x56395eabb1a8 - main
  26:     0x7f016c699d90 - <unknown>
  27:     0x7f016c699e40 - __libc_start_main
  28:     0x56395eabb065 - _start
  29:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (121cee6eb 2022-12-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options -Z force-unstable-if-unmarked
query stack during panic:
end of query stack
end of query stack
thread '<unnamed>' panicked at 'assertion failed: Rc::is_unique(this)', /checkout/library/alloc/src/rc.rs:1148:9
   0:     0x7f016c95fe32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   0:     0x7f016c95fe32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   1:     0x7f016c9cf078 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7f016c9508c1 - std::io::Write::write_fmt::h115d78592252bf9d
   3:     0x7f016c95fbf5 - std::sys_common::backtrace::print::hd9ebfd55eb3388eb
   4:     0x7f016c963017 - std::panicking::default_hook::{{closure}}::h98850a6457a8782d
   5:     0x7f016c962cdb - std::panicking::default_hook::hef750a0d73d5e8e9
   6:     0x7f016d34a894 - rustc_driver[6405d95162ec2db2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f016c963923 - std::panicking::rust_panic_with_hook::ha62dd9a4e901383e
   8:     0x7f016c963611 - std::panicking::begin_panic_handler::{{closure}}::h30577589484d4d4f
   9:     0x7f016c9603dc - std::sys_common::backtrace::__rust_end_short_backtrace::h8641e565b807163f
  10:     0x7f016c963322 - rust_begin_unwind
  11:     0x7f016c914033 - core::panicking::panic_fmt::ha29375e42f9e82fb
  12:     0x7f016c91410d - core::panicking::panic::hb659ada2b02be8b4
  13:     0x7f016d3af5a7 - <alloc[6d250edaf69a7784]::rc::Rc<rustc_ast[6ac300baecbcce0c]::ast::Crate> as core[867cfca19013d5a]::ops::drop::Drop>::drop
  14:     0x7f016d33ff1c - core[867cfca19013d5a]::ptr::drop_in_place::<rustc_errors[6d9bb464f3a9d055]::Handler>
  15:     0x7f016d34b96e - rustc_driver[6405d95162ec2db2]::report_ice
  16:     0x7f016c963923 - std::panicking::rust_panic_with_hook::ha62dd9a4e901383e
  17:     0x7f016c963611 - std::panicking::begin_panic_handler::{{closure}}::h30577589484d4d4f
  18:     0x7f016c9603dc - std::sys_common::backtrace::__rust_end_short_backtrace::h8641e565b807163f
  19:     0x7f016c963322 - rust_begin_unwind
  20:     0x7f016c914033 - core::panicking::panic_fmt::ha29375e42f9e82fb
  21:     0x7f016c91410d - core::panicking::panic::hb659ada2b02be8b4
  22:     0x7f016c927a16 - alloc::sync::Arc<T>::drop_slow::h63351216a29d5be0
  23:     0x7f016c975393 - std::sys_common::thread_info::THREAD_INFO::__getit::destroy::h4eddde57fd78359c
  24:     0x7f016c6b5d9f - __call_tls_dtors
  25:     0x7f016c6b55c9 - <unknown>
  26:     0x7f016c6b5610 - exit
  27:     0x7f016c96c857 - std::sys::unix::os::exit::h34004398f9017db1
  28:     0x7f016c95e9af - std::process::exit::h65ff648367d53ca3
  29:     0x7f016d34c097 - rustc_driver[6405d95162ec2db2]::main
  30:     0x56395eabb177 - rustc_main[2dd750e4836bcfb6]::main
  31:     0x56395eabb133 - std[65b05b99172e1c7e]::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  32:     0x56395eabb159 - std[65b05b99172e1c7e]::rt::lang_start::<()>::{closure#0}
  33:     0x7f016c93ea24 - std::rt::lang_start_internal::hafd522f3051e9319
  34:     0x56395eabb1a8 - main
  35:     0x7f016c699d90 - <unknown>
  36:     0x7f016c699e40 - __libc_start_main
  37:     0x56395eabb065 - _start
  38:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (121cee6eb 2022-12-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options -Z force-unstable-if-unmarked
query stack during panic:
end of query stack
thread panicked while processing panic. aborting.
thread panicked while processing panic. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)
