plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_hir_analysis/src/astconv/mod.rs at line 1320:
                 let mut append_no = 2;
                 while regular_traits_vec.contains(&format!("NewTrait{append_no}")) {
                     append_no += 1;
+                }
+                }
                 format!("NewTrait{append_no}")
             } else {
                 "NewTrait".to_string()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/thir/pattern/const_to_pat.rs" "/checkout/compiler/rustc_hir_analysis/src/collect/item_bounds.rs" "/checkout/compiler/rustc_hir_analysis/src/astconv/mod.rs" "/checkout/compiler/rustc_hir_analysis/src/errors.rs" "/checkout/compiler/rustc_hir_analysis/src/astconv/errors.rs" "/checkout/compiler/rustc_hir_analysis/src/astconv/generics.rs" "/checkout/compiler/rustc_hir_analysis/src/structured_errors/wrong_number_of_generic_args.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
