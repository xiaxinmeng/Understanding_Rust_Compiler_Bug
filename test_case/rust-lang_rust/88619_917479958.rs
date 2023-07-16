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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/os/mod.rs at line 53:
 pub mod windows {}
 
 // unix
-#[cfg(not(all(doc, any(
-    all(target_arch = "wasm32", not(target_os = "wasi")),
-    all(target_vendor = "fortanix", target_env = "sgx")
-))))]
+#[cfg(not(all(
+    doc,
+    any(
+        all(target_arch = "wasm32", not(target_os = "wasi")),
+        all(target_vendor = "fortanix", target_env = "sgx")
+)))]
+)))]
 #[cfg(target_os = "hermit")]
 #[path = "hermit/mod.rs"]
 pub mod unix;
Diff in /checkout/library/std/src/os/mod.rs at line 63:
-#[cfg(not(all(doc, any(
-    all(target_arch = "wasm32", not(target_os = "wasi")),
-    all(target_vendor = "fortanix", target_env = "sgx")
-))))]
+#[cfg(not(all(
+    doc,
+    any(
+        all(target_arch = "wasm32", not(target_os = "wasi")),
+        all(target_vendor = "fortanix", target_env = "sgx")
+)))]
+)))]
 #[cfg(all(not(target_os = "hermit"), any(unix, doc)))]
 pub mod unix;
Diff in /checkout/library/std/src/os/mod.rs at line 70:
 // linux
 // linux
-#[cfg(not(all(doc, any(
-    all(target_arch = "wasm32", not(target_os = "wasi")),
-    all(target_vendor = "fortanix", target_env = "sgx")
-))))]
+#[cfg(not(all(
+    doc,
+    any(
+        all(target_arch = "wasm32", not(target_os = "wasi")),
+        all(target_vendor = "fortanix", target_env = "sgx")
+)))]
+)))]
 #[cfg(any(target_os = "linux", target_os = "l4re", doc))]
 pub mod linux;
Diff in /checkout/library/std/src/os/mod.rs at line 78:
 // wasi
 // wasi
-#[cfg(not(all(doc, any(
-    all(target_arch = "wasm32", not(target_os = "wasi")),
-    all(target_vendor = "fortanix", target_env = "sgx")
-))))]
+#[cfg(not(all(
+    doc,
+    any(
+        all(target_arch = "wasm32", not(target_os = "wasi")),
+        all(target_vendor = "fortanix", target_env = "sgx")
+)))]
+)))]
 #[cfg(any(target_os = "wasi", doc))]
 pub mod wasi;
Diff in /checkout/library/std/src/os/mod.rs at line 86:
 // windows
 // windows
-#[cfg(not(all(doc, any(
-    all(target_arch = "wasm32", not(target_os = "wasi")),
-    all(target_vendor = "fortanix", target_env = "sgx")
-))))]
+#[cfg(not(all(
+    doc,
+    any(
+        all(target_arch = "wasm32", not(target_os = "wasi")),
+        all(target_vendor = "fortanix", target_env = "sgx")
+)))]
+)))]
 #[cfg(any(windows, doc))]
 pub mod windows;
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/fd/net.rs" "/checkout/library/std/src/os/fortanix_sgx/io.rs" "/checkout/library/std/src/os/fd/owned.rs" "/checkout/library/std/src/os/fortanix_sgx/ffi.rs" "/checkout/library/std/src/os/fd/mod.rs" "/checkout/library/std/src/os/fd/raw.rs" "/checkout/library/std/src/os/netbsd/raw.rs" "/checkout/library/std/src/os/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
