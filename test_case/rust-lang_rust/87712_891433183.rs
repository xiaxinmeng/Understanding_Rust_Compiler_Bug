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
Diff in /checkout/library/proc_macro/src/lib.rs at line 440:
 
 impl LineColumn {
     fn add_1_to_column(self) -> Self {
-        LineColumn {
-            line: self.line,
-            column: self.column + 1,
-        }
+        LineColumn { line: self.line, column: self.column + 1 }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/panic_unwind/src/lib.rs" "/checkout/library/panic_unwind/src/emcc.rs" "/checkout/library/panic_unwind/src/seh.rs" "/checkout/library/proc_macro/src/lib.rs" "/checkout/library/panic_unwind/src/gcc.rs" "/checkout/library/proc_macro/src/quote.rs" "/checkout/library/panic_unwind/src/dummy.rs" "/checkout/library/proc_macro/src/bridge/closure.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
