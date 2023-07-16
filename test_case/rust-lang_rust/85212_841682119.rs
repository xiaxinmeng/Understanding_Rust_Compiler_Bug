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
Diff in /checkout/compiler/rustc_parse/src/tests.rs at line 23:
         #[bench]
         fn $name(b: &mut Bencher) {
             let tc = black_box(mk_token_cursor($n));
-            b.iter(|| { let _ = tc.clone(); });
+            b.iter(|| {
+                let _ = tc.clone();
+            });
     };
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/item.rs" "/checkout/compiler/rustc_parse/src/tests.rs" "/checkout/compiler/rustc_parse/src/lib.rs" "/checkout/compiler/rustc_macros/src/query.rs" "/checkout/compiler/rustc_macros/src/symbols.rs" "/checkout/compiler/rustc_macros/src/lift.rs" "/checkout/compiler/rustc_macros/src/type_foldable.rs" "/checkout/compiler/rustc_parse/src/parser/pat.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
