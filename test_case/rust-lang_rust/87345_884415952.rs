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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/tools/tidy/src/deps.rs at line 26:
 /// tooling. It is _crucial_ that no exception crates be dependencies
 /// of the Rust runtime (std/test).
 const EXCEPTIONS: &[(&str, &str)] = &[
-    ("mdbook", "MPL-2.0"),              // mdbook
-    ("openssl", "Apache-2.0"),          // cargo, mdbook
-    ("colored", "MPL-2.0"),             // rustfmt
-    ("ordslice", "Apache-2.0"),         // rls
-    ("ryu", "Apache-2.0 OR BSL-1.0"),   // rls/cargo/... (because of serde)
-    ("bytesize", "Apache-2.0"),         // cargo
-    ("im-rc", "MPL-2.0+"),              // cargo
-    ("sized-chunks", "MPL-2.0+"),       // cargo via im-rc
-    ("bitmaps", "MPL-2.0+"),            // cargo via im-rc
+    ("mdbook", "MPL-2.0"),                                  // mdbook
+    ("openssl", "Apache-2.0"),                              // cargo, mdbook
+    ("colored", "MPL-2.0"),                                 // rustfmt
+    ("ordslice", "Apache-2.0"),                             // rls
+    ("ryu", "Apache-2.0 OR BSL-1.0"),                       // rls/cargo/... (because of serde)
+    ("bytesize", "Apache-2.0"),                             // cargo
+    ("im-rc", "MPL-2.0+"),                                  // cargo
+    ("sized-chunks", "MPL-2.0+"),                           // cargo via im-rc
+    ("bitmaps", "MPL-2.0+"),                                // cargo via im-rc
     ("crossbeam-queue", "MIT/Apache-2.0 AND BSD-2-Clause"), // rls via rayon
-    ("instant", "BSD-3-Clause"),        // rustc_driver/tracing-subscriber/parking_lot
-    ("snap", "BSD-3-Clause"),           // rustc
+    ("instant", "BSD-3-Clause"), // rustc_driver/tracing-subscriber/parking_lot
+    ("snap", "BSD-3-Clause"),    // rustc
     // FIXME: this dependency violates the documentation comment above:
     ("fortanix-sgx-abi", "MPL-2.0"), // libstd but only for `sgx` target
 ];
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tidy/src/lib.rs" "/checkout/src/tools/tidy/src/extdeps.rs" "/checkout/src/tools/tidy/src/features.rs" "/checkout/src/tools/tidy/src/errors.rs" "/checkout/src/tools/tidy/src/unit_tests.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/tools/tidy/src/deps.rs" "/checkout/src/tools/tidy/src/target_specific_tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
