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
Diff in /checkout/library/std/src/os/unix/net/tests.rs at line 665:
     }
 }
 
-#[cfg(any(
-    target_os = "android",
-    target_os = "emscripten",
-    target_os = "linux",
-))]
+#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "linux",))]
 #[test]
 fn test_send_vectored_with_ancillary_to_unix_datagram() {
     fn getpid() -> libc::pid_t {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/solaris/mod.rs" "/checkout/library/std/src/os/solaris/raw.rs" "/checkout/library/std/src/os/unix/net/addr.rs" "/checkout/library/std/src/os/unix/net/tests.rs" "/checkout/library/core/src/fmt/num.rs" "/checkout/library/std/src/os/android/fs.rs" "/checkout/library/std/src/os/android/mod.rs" "/checkout/library/std/src/os/solaris/fs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
