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
         })
     }
 
-
     // Calls `f` with the internal `let_expr_allowed` set to `let_expr_allowed` and then
     // sets the internal `let_expr_allowed` back to its original value.
     fn with_let_management<T>(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/attr.rs" "/checkout/compiler/rustc_incremental/src/assert_module_sources.rs" "/checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs" "/checkout/compiler/rustc_parse/src/parser/pat.rs" "/checkout/compiler/rustc_parse/src/parser/expr.rs" "/checkout/compiler/rustc_incremental/src/persist/file_format.rs" "/checkout/compiler/rustc_incremental/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
