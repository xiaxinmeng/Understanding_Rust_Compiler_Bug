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
Diff in /checkout/library/std/src/lib.rs at line 193:
     html_playground_url = "https://play.rust-lang.org/",
     issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
     test(no_crate_inject, attr(deny(warnings))),
-    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))),
+    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
 )]
-#![cfg_attr(not(bootstrap),
-    doc(cfg_hide(not(test), not(any(test, bootstrap))))
-)]
+#![cfg_attr(not(bootstrap), doc(cfg_hide(not(test), not(any(test, bootstrap)))))]
 // Don't link to std. We are std.
 #![no_std]
 #![warn(deprecated_in_future)]
Diff in /checkout/library/core/src/lib.rs at line 58:
     html_playground_url = "https://play.rust-lang.org/",
     issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
     test(no_crate_inject, attr(deny(warnings))),
-    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))),
+    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
 )]
-#![cfg_attr(not(bootstrap),
+#![cfg_attr(
+    not(bootstrap),
     doc(cfg_hide(
         not(test),
         target_pointer_width = "16",
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/adapters/fuse.rs" "/checkout/library/core/src/iter/traits/iterator.rs" "/checkout/library/core/src/iter/adapters/take_while.rs" "/checkout/library/core/src/iter/traits/mod.rs" "/checkout/library/core/src/iter/mod.rs" "/checkout/library/core/src/lib.rs" "/checkout/library/core/src/slice/ascii.rs" "/checkout/library/core/src/iter/adapters/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
