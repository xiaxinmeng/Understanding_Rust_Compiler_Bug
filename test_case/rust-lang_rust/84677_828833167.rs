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
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0432]: unresolved import `crate::net::test`
 --> library/std/src/net/addr/tests.rs:1:17
  |
1 | use crate::net::test::{sa4, sa6, tsa};
  |                 ^^^^ could not find `test` in `net`
error[E0432]: unresolved import `crate::net::test`
 --> library/std/src/net/ip/tests.rs:1:17
  |
  |
1 | use crate::net::test::{sa4, sa6, tsa};
  |                 ^^^^ could not find `test` in `net`
error[E0432]: unresolved import `crate::net::test`
 --> library/std/src/net/tcp/tests.rs:4:17
  |
  |
4 | use crate::net::test::{next_test_ip4, next_test_ip6};
  |                 ^^^^ could not find `test` in `net`
error[E0432]: unresolved import `crate::net::test`
 --> library/std/src/net/udp/tests.rs:2:17
  |
  |
2 | use crate::net::test::{next_test_ip4, next_test_ip6};
  |                 ^^^^ could not find `test` in `net`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0432`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "core" "-p" "proc_macro" "-p" "panic_abort" "-p" "alloc" "-p" "term" "-p" "std" "-p" "unwind" "-p" "panic_unwind" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:39
