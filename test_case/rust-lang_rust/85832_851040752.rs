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
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0277]: a value of type `realstd::vec::Vec<ffi::os_str::OsString>` cannot be built from an iterator over elements of type `Arg`
  --> library/std/src/sys/windows/process/tests.rs:29:66
   |
29 |             &args.iter().map(|a| Arg::Quoted(OsString::from(a))).collect::<Vec<OsString>>(),
   |                                                                  ^^^^^^^ value of type `realstd::vec::Vec<ffi::os_str::OsString>` cannot be built from `std::iter::Iterator<Item=Arg>`
   |
   = help: the trait `FromIterator<Arg>` is not implemented for `realstd::vec::Vec<ffi::os_str::OsString>`
error[E0308]: mismatched types
  --> library/std/src/sys/windows/process/tests.rs:29:13
   |
   |
29 |             &args.iter().map(|a| Arg::Quoted(OsString::from(a))).collect::<Vec<OsString>>(),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice, found struct `realstd::vec::Vec`
   |
   = note: expected reference `&[Arg]`
              found reference `&realstd::vec::Vec<ffi::os_str::OsString>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_abort" "-p" "alloc" "-p" "proc_macro" "-p" "panic_unwind" "-p" "unwind" "-p" "std" "-p" "term" "-p" "core" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:28
