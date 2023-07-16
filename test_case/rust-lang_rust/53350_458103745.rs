
error[E0080]: constant evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
   --> /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/cmp.rs:947:52
    |
947 |                 fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
    |                                                    ^^^^^^^^^^^^^^^^^^^ attempted to do invalid arithmetic on pointers that woulerror[E0080]: constant evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
   --> /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/cmp.rs:947:52
    |
947 |                 fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
    |                                                    ^^^^^^^^^^^^^^^^^^^ attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
    |
    = note: inside call to `std::cmp::impls::<impl std::cmp::PartialOrd for usize>::ge` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/num.rs:55:30
    = note: inside call to `<core::fmt::num::LowerHex as core::fmt::num::GenericRadix>::fmt_int::<usize>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/num.rs:133:46
    = note: inside call to `core::fmt::num::<impl std::fmt::LowerHex for usize>::fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/mod.rs:2001:19
    = note: inside call to `<*const T as std::fmt::Pointer><[i32; 3]>::fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/mod.rs:2020:9
    = note: inside call to `<&T as std::fmt::Pointer><[i32; 3]>::fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/mod.rs:1016:17
    = note: inside call to `std::fmt::write` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/mod.rs:1130:15
    = note: inside call to `<std::io::StdoutLock as std::io::Write>::write_fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/stdio.rs:454:9
    = note: inside call to `<std::io::Stdout as std::io::Write>::write_fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/stdio.rs:684:9
    = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/thread/local.rs:296:16
    = note: inside call to `<std::thread::LocalKey<T>><std::cell::RefCell<std::option::Option<std::boxed::Box<dyn std::io::Write + std::marker::Send>>>>::try_with::<[closure@DefId(1/1:1090 ~ std[82ff]::io[0]::stdio[0]::print_to[0]::{{closure}}[0]) 0:&std::fmt::Arguments, 1:&fn() -> std::io::Stdout], std::result::Result<(), std::io::Error>>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/stdio.rs:678:18
    = note: inside call to `std::io::stdio::print_to::<std::io::Stdout>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/stdio.rs:699:5
note: inside call to `std::io::_print` at <::std::macros::println macros>:2:3
   --> src/main.rs:4:5
    |
4   |     println!("{:p}", &a);  // 0x7ffcbc067704
    |     ^^^^^^^^^^^^^^^^^^^^^
    = note: inside call to `main` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:64:34
    = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:52:53
    = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:297:40
    = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1/1:1899 ~ std[82ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:293:5
    = note: inside call to `std::panicking::try::<i32, [closure@DefId(1/1:1899 ~ std[82ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panic.rs:388:9
    = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1/1:1899 ~ std[82ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:52:25
    = note: inside call to `std::rt::lang_start_internal` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:64:5
    = note: inside call to `std::rt::lang_start::<()>`
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
d leak base addresses, e.g., comparing pointers into different allocations
    |
    = note: inside call to `std::cmp::impls::<impl std::cmp::PartialOrd for usize>::ge` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/num.rs:55:30
    = note: inside call to `<core::fmt::num::LowerHex as core::fmt::num::GenericRadix>::fmt_int::<usize>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/num.rs:133:46
    = note: inside call to `core::fmt::num::<impl std::fmt::LowerHex for usize>::fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/mod.rs:2001:19
    = note: inside call to `<*const T as std::fmt::Pointer><[i32; 3]>::fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/mod.rs:2020:9
    = note: inside call to `<&T as std::fmt::Pointer><[i32; 3]>::fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/fmt/mod.rs:1016:17
    = note: inside call to `std::fmt::write` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/mod.rs:1130:15
    = note: inside call to `<std::io::StdoutLock as std::io::Write>::write_fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/stdio.rs:454:9
    = note: inside call to `<std::io::Stdout as std::io::Write>::write_fmt` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/stdio.rs:684:9
    = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/thread/local.rs:296:16
    = note: inside call to `<std::thread::LocalKey<T>><std::cell::RefCell<std::option::Option<std::boxed::Box<dyn std::io::Write + std::marker::Send>>>>::try_with::<[closure@DefId(1/1:1090 ~ std[82ff]::io[0]::stdio[0]::print_to[0]::{{closure}}[0]) 0:&std::fmt::Arguments, 1:&fn() -> std::io::Stdout], std::result::Result<(), std::io::Error>>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/stdio.rs:678:18
    = note: inside call to `std::io::stdio::print_to::<std::io::Stdout>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/stdio.rs:699:5
note: inside call to `std::io::_print` at <::std::macros::println macros>:2:3
   --> src/main.rs:4:5
    |
4   |     println!("{:p}", &a);  // 0x7ffcbc067704
    |     ^^^^^^^^^^^^^^^^^^^^^
    = note: inside call to `main` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:64:34
    = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:52:53
    = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:297:40
    = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1/1:1899 ~ std[82ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:293:5
    = note: inside call to `std::panicking::try::<i32, [closure@DefId(1/1:1899 ~ std[82ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panic.rs:388:9
    = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1/1:1899 ~ std[82ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:52:25
    = note: inside call to `std::rt::lang_start_internal` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:64:5
    = note: inside call to `std::rt::lang_start::<()>`
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

