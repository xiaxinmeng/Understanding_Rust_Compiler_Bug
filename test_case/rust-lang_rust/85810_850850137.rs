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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_driver/src/lib.rs at line 1068:
 
     if cg_flags.iter().any(|x| *x == "passes=list") {
         let backend_name = debug_flags.iter().find_map(|x| {
-            if x.starts_with("codegen-backend=") { Some(&x["codegen-backends=".len()..]) } else { None }
+            if x.starts_with("codegen-backend=") {
+                Some(&x["codegen-backends=".len()..])
+                None
+            }
         });
         });
         get_codegen_backend(&None, backend_name).print_passes();
         return None;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/term/src/terminfo/searcher/tests.rs" "/checkout/library/term/src/terminfo/parser/compiled.rs" "/checkout/library/term/src/terminfo/parser/compiled/tests.rs" "/checkout/library/term/src/terminfo/parm.rs" "/checkout/compiler/rustc_driver/src/lib.rs" "/checkout/library/term/src/terminfo/mod.rs" "/checkout/compiler/rustc_driver/src/pretty.rs" "/checkout/library/term/src/terminfo/searcher.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
