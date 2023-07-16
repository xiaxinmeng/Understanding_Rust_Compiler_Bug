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
    Checking core v0.0.0 (/checkout/library/core)
error: trailing semicolon in macro used in expression position
  --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:21:65
   |
14 | /         macro_rules! $macro_name {
15 | |             $(
16 | |                 ($feature_lit) => {
17 | |                     $crate::detect::__is_feature_detected::$feature()
...  |
21 | |                 ($bind_feature) => { $macro_name!($feature_impl); };
...  |
43 | |             };
44 | |         }
44 | |         }
   | |_________- in this expansion of `is_x86_feature_detected!`
  ::: library/std/tests/run-time-detect.rs:60:27
   |
   |
60 |       println!("abm: {:?}", is_x86_feature_detected!("abm")); // this is a synonym for lzcnt but we test it anyways
   |
   |
   = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>

error: aborting due to previous error


error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "term" "-p" "proc_macro" "-p" "core" "-p" "panic_abort" "-p" "alloc" "-p" "panic_unwind" "-p" "unwind" "-p" "std" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:44
