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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_ast_passes/src/errors.rs at line 26:
             }
             Self::NotSupportedParentheses(span) => {
                 diag.span_note(span, fluent::ast_passes::not_supported_parentheses);
+            }
         }
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_passes/src/node_count.rs" "/checkout/compiler/rustc_ast_passes/src/show_span.rs" "/checkout/compiler/rustc_smir/src/very_unstable.rs" "/checkout/compiler/rustc_smir/src/lib.rs" "/checkout/compiler/rustc_smir/src/mir.rs" "/checkout/compiler/rustc_borrowck/src/borrow_set.rs" "/checkout/compiler/rustc_borrowck/src/nll.rs" "/checkout/compiler/rustc_ast_passes/src/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
