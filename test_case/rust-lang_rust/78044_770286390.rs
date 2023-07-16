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
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
error: use of associated function `core::num::<impl u64>::max_value` that will be deprecated in a future Rust version: replaced by the `MAX` associated constant on this type
  --> library/std/src/io/util/tests.rs:37:45
   |
37 |     assert!(matches!(e.seek(SeekFrom::Start(u64::max_value())), Ok(0)));
   |
   |
   = note: `-D deprecated-in-future` implied by `-D warnings`

error: use of associated function `core::num::<impl i64>::min_value` that will be deprecated in a future Rust version: replaced by the `MIN` associated constant on this type
  --> library/std/src/io/util/tests.rs:39:43
   |
39 |     assert!(matches!(e.seek(SeekFrom::End(i64::min_value())), Ok(0)));


error: use of associated function `core::num::<impl i64>::max_value` that will be deprecated in a future Rust version: replaced by the `MAX` associated constant on this type
  --> library/std/src/io/util/tests.rs:43:43
   |
43 |     assert!(matches!(e.seek(SeekFrom::End(i64::max_value())), Ok(0)));


error: use of associated function `core::num::<impl i64>::min_value` that will be deprecated in a future Rust version: replaced by the `MIN` associated constant on this type
  --> library/std/src/io/util/tests.rs:45:47
   |
45 |     assert!(matches!(e.seek(SeekFrom::Current(i64::min_value())), Ok(0)));


error: use of associated function `core::num::<impl i64>::max_value` that will be deprecated in a future Rust version: replaced by the `MAX` associated constant on this type
  --> library/std/src/io/util/tests.rs:49:47
   |
49 |     assert!(matches!(e.seek(SeekFrom::Current(i64::max_value())), Ok(0)));

error: aborting due to 5 previous errors

error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "std" "-p" "unwind" "-p" "alloc" "-p" "proc_macro" "-p" "core" "-p" "panic_unwind" "-p" "panic_abort" "-p" "term" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:42
