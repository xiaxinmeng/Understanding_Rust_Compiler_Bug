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
Diff in /checkout/compiler/rustc_mir_build/src/thir/constant.rs at line 20:
         let result = match &ty.kind() {
             ty::Uint(_) => {
                 let max_value = width.unsigned_int_max();
-                if n >= max_value {
-                } else {
-                } else {
-                    width.truncate(n)
-                }
+                if n >= max_value { max_value } else { width.truncate(n) }
             }
             _ => width.truncate(n),
         };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/build/mod.rs" "/checkout/compiler/rustc_mir_build/src/build/misc.rs" "/checkout/compiler/rustc_mir_build/src/build/cfg.rs" "/checkout/compiler/rustc_mir_build/src/check_unsafety.rs" "/checkout/compiler/rustc_mir_build/src/build/scope.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/stmt.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/category.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
