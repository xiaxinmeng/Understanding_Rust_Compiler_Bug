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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 2378:
         let filename = cratepath.file_name().unwrap().to_str().unwrap();
         let symbol_name = get_dylib_symbol_name(filename, &sess.target)
             .unwrap_or(unlib(&sess.target, cratepath.file_stem().unwrap().to_str().unwrap()));
-        cmd.link_rust_dylib(
-            Symbol::intern(symbol_name),
-            cratepath,
-        );
+        cmd.link_rust_dylib(Symbol::intern(symbol_name), cratepath);
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/archive.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/target_features.rs" "/checkout/compiler/rustc_session/src/output.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
