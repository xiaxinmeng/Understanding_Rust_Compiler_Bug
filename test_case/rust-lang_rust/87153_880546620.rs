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
Diff in /checkout/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs at line 601:
     // as part of a cast like the `HashMap` visualizer does.
     if cpp_like_names {
         output.push(',');
-    }else {
+    } else {
         output.push_str(", ");
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/mir/intrinsic.rs" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/analyze.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/debuginfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/statement.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/coverageinfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/coverageinfo/ffi.rs" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
