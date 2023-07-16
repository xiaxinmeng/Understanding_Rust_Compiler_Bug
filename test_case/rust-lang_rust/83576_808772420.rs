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
Diff in /checkout/library/core/tests/array.rs at line 309:
 #[test]
 fn array_from_fn() {
     let mut i = 0;
-    let a = array::from_fn(|| { i += 1; i });
+    let a = array::from_fn(|| {
+        i += 1;
+    });
+    });
     assert_eq!(a, [1, 2, 3, 4]);
 
     let b = array::from_fn(|| String::from("foo"));
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/result.rs" "/checkout/library/core/tests/char.rs" "/checkout/library/core/tests/cell.rs" "/checkout/library/core/tests/option.rs" "/checkout/library/unwind/src/libunwind.rs" "/checkout/library/core/tests/macros.rs" "/checkout/library/core/tests/array.rs" "/checkout/library/core/tests/tuple.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
