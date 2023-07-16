plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0658]: use of unstable library feature 'int_log'
 --> library/core/tests/num/int_log.rs:3:23
  |
3 |     assert_eq!(999u32.checked_log(10), Some(2));
  |
  = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
  = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
 --> library/core/tests/num/int_log.rs:4:24
  |
4 |     assert_eq!(1000u32.checked_log(10), Some(3));
  |
  = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
  = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
 --> library/core/tests/num/int_log.rs:5:23
  |
5 |     assert_eq!(555u32.checked_log(13), Some(2));
  |
  = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
  = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
 --> library/core/tests/num/int_log.rs:6:22
  |
6 |     assert_eq!(63u32.checked_log(4), Some(2));
  |
  = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
  = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
 --> library/core/tests/num/int_log.rs:7:22
  |
7 |     assert_eq!(64u32.checked_log(4), Some(3));
  |
  = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
  = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
 --> library/core/tests/num/int_log.rs:8:31
  |
8 |     assert_eq!(10460353203u64.checked_log(3), Some(21));
  |
  = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
  = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
 --> library/core/tests/num/int_log.rs:9:31
  |
9 |     assert_eq!(10460353202u64.checked_log(3), Some(20));
  |
  = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
  = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:10:60
   |
10 |     assert_eq!(147808829414345923316083210206383297601u128.checked_log(3), Some(80));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:11:60
   |
11 |     assert_eq!(147808829414345923316083210206383297600u128.checked_log(3), Some(79));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:12:56
   |
12 |     assert_eq!(22528399544939174411840147874772641u128.checked_log(19683), Some(8));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:13:56
   |
13 |     assert_eq!(22528399544939174411840147874772631i128.checked_log(19683), Some(7));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:16:22
   |
16 |         assert_eq!(i.checked_log(4), None);
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:19:22
   |
19 |         assert_eq!(i.checked_log(13), Some((i as f32).log(13.0) as i16));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:22:22
   |
22 |         assert_eq!(i.checked_log(13), Some((i as f32).log(13.0) as u16));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:28:21
   |
28 |     assert_eq!(5u32.checked_log2(), Some(2));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:29:21
   |
29 |     assert_eq!(0u64.checked_log2(), None);
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:30:23
   |
30 |     assert_eq!(128i32.checked_log2(), Some(7));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:31:25
   |
31 |     assert_eq!((-55i16).checked_log2(), None);
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:34:22
   |
34 |         assert_eq!(i.checked_log2(), Some((i as f32).log2() as u8));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:37:22
   |
37 |         assert_eq!(i.checked_log2(), Some((i as f32).log2() as u16));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:40:22
   |
40 |         assert_eq!(i.checked_log2(), None);
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:43:22
   |
43 |         assert_eq!(i.checked_log2(), Some((i as f32).log2() as i8));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:46:22
   |
46 |         assert_eq!(i.checked_log2(), None);
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:49:22
   |
49 |         assert_eq!(i.checked_log2(), Some((i as f32).log2() as i16));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:56:22
   |
56 |         assert_eq!(i.checked_log10(), None);
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:59:22
   |
59 |         assert_eq!(i.checked_log10(), Some((i as f32).log10() as i16));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_log'
  --> library/core/tests/num/int_log.rs:62:22
   |
62 |         assert_eq!(i.checked_log10(), Some((i as f32).log10() as u16));
   |
   = note: see issue #70887 <https://github.com/rust-lang/rust/issues/70887> for more information
   = help: add `#![feature(int_log)]` to the crate attributes to enable

---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_abort" "-p" "core" "-p" "term" "-p" "panic_unwind" "-p" "alloc" "-p" "unwind" "-p" "proc_macro" "-p" "std" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:44
