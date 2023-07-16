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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/different_lifetimes.rs at line 236:
                             Applicability::MaybeIncorrect,
                         );
                         );
-                        e.note("Each elided lifetime in input position becomes a distinct lifetime.");
+                        e.note(
+                            "Each elided lifetime in input position becomes a distinct lifetime.",
                     }
                 }
             }
Build completed unsuccessfully in 0:00:15
Build completed unsuccessfully in 0:00:15
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/named_anon_conflict.rs" "/checkout/compiler/rustc_infer/src/infer/region_constraints/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/placeholder_error.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/find_anon_type.rs" "/checkout/compiler/rustc_infer/src/infer/region_constraints/leak_check.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/different_lifetimes.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/trait_impl_difference.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
