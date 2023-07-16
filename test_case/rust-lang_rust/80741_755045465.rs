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
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: use `std::fs::metadata` instead
   --> library/std/src/fs/tests.rs:738:43
    |
738 |     assert_eq!(check!(tmp.join("out.txt").metadata()).len(), 0);
    |                                           ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`
    |
    = note: `-D deprecated-in-future` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `std`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "std" "-p" "term" "-p" "panic_abort" "-p" "proc_macro" "-p" "core" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:44
