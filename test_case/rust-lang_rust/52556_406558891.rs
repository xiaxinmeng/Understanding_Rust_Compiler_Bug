plain
[00:04:13]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:14] error[E0432]: unresolved import `core::time::M`
[00:04:14]   --> libstd/time.rs:34:37
[00:04:14]    |
[00:04:14] 34 | pub use core::time::{NS, US, MS, S, M, H, D};
[00:04:14]    |                                     ^ no `M` in `time`. Did you mean to use `MS`?
[00:04:14] error[E0432]: unresolved import `core::time::H`
[00:04:14]   --> libstd/time.rs:34:40
[00:04:14]    |
[00:04:14]    |
[00:04:14] 34 | pub use core::time::{NS, US, MS, S, M, H, D};
[00:04:14]    |                                        ^ no `H` in `time`. Did you mean to use `S`?
[00:04:14] error[E0432]: unresolved import `core::time::D`
[00:04:14]   --> libstd/time.rs:34:43
[00:04:14]    |
[00:04:14]    |
[00:04:14] 34 | pub use core::time::{NS, US, MS, S, M, H, D};
[00:04:14]    |                                           ^ no `D` in `time`. Did you mean to use `S`?
[00:04:14] error[E0658]: use of unstable library feature 'time_units'
[00:04:14]   --> libstd/time.rs:34:22
[00:04:14]    |
[00:04:14]    |
[00:04:14] 34 | pub use core::time::{NS, US, MS, S, M, H, D};
[00:04:14]    |
[00:04:14]    |
[00:04:14]    = help: add #![feature(time_units)] to the crate attributes to enable
[00:04:14] error[E0658]: use of unstable library feature 'time_units'
[00:04:14]   --> libstd/time.rs:34:26
[00:04:14]    |
[00:04:14]    |
[00:04:14] 34 | pub use core::time::{NS, US, MS, S, M, H, D};
[00:04:14]    |
[00:04:14]    |
[00:04:14]    = help: add #![feature(time_units)] to the crate attributes to enable
[00:04:14] error[E0658]: use of unstable library feature 'time_units'
[00:04:14]   --> libstd/time.rs:34:30
[00:04:14]    |
[00:04:14]    |
[00:04:14] 34 | pub use core::time::{NS, US, MS, S, M, H, D};
[00:04:14]    |
[00:04:14]    |
[00:04:14]    = help: add #![feature(time_units)] to the crate attributes to enable
[00:04:14] error[E0658]: use of unstable library feature 'time_units'
[00:04:14]   --> libstd/time.rs:34:34
[00:04:14]    |
[00:04:14]    |
[00:04:14] 34 | pub use core::time::{NS, US, MS, S, M, H, D};
[00:04:14]    |
[00:04:14]    |
[00:04:14]    = help: add #![feature(time_units)] to the crate attributes to enable
[00:04:16] error: aborting due to 7 previous errors
[00:04:16] 
[00:04:16] Some errors occurred: E0432, E0658.
[00:04:16] For more information about an error, try `rustc --explain E0432`.
[00:04:16] For more information about an error, try `rustc --explain E0432`.
[00:04:16] error: Could not compile `std`.
[00:04:16] 
[00:04:16] Caused by:
[00:04:16]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=38eda8139e25afc6 -C extra-filename=-38eda8139e25afc6 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-0fa369be6843d38b.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-d44ab1cdae3c5d5f.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-61125bfca7e472f5.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-b64848753b7b1fae.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-13801a3823f081b4.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-6b8b73430c4ddfaf.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-6dd09c3866f99ef9.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-8ca8a77f92241276.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-c00ded6ab9dc527a.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0045e7e201e984fa.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-8f8910030c71780d.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-06bfec7617605ba9.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-ea8c3e35572b37f6.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-ff87cc4f36401518.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -L nati
