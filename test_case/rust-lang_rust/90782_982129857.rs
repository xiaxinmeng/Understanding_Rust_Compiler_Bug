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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_llvm/src/back/archive.rs at line 176:
             .collect();
 
         if mingw_gnu_toolchain {
-            let def_file_path = tmpdir.as_ref().join(format!("{}_imports", lib_name)).with_extension("def");
+            let def_file_path =
+                tmpdir.as_ref().join(format!("{}_imports", lib_name)).with_extension("def");
             let def_file_content = format!(
             let def_file_content = format!(
                 "EXPORTS\n{}",
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/back/profiling.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/lto.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/write.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/archive.rs" "/checkout/compiler/rustc_borrowck/src/consumers.rs" "/checkout/compiler/rustc_codegen_llvm/src/mono_item.rs" "/checkout/compiler/rustc_borrowck/src/dataflow.rs" "/checkout/compiler/rustc_borrowck/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
