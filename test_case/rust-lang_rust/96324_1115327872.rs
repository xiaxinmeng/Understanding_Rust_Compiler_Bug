plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/os/linux/tests.rs at line 1:
 use crate::fmt;
 use crate::net::test::next_test_ip4;
 use crate::net::{TcpListener, TcpStream};
-use crate::os::linux::net::{TcpStreamExt};
+use crate::os::linux::net::TcpStreamExt;
 macro_rules! t {
     ($e:expr) => {
     ($e:expr) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/linux/mod.rs" "/checkout/library/std/src/os/linux/net.rs" "/checkout/library/std/src/os/linux/fs.rs" "/checkout/library/std/src/os/linux/tests.rs" "/checkout/library/std/src/os/espidf/raw.rs" "/checkout/library/std/src/error/tests.rs" "/checkout/library/std/src/lib.rs" "/checkout/library/std/src/os/linux/process.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
