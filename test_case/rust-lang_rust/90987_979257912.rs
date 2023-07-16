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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 855:
             .regions()
             .map(|lifetime| {
                 let s = lifetime.to_string();
-                if s.is_empty() {
-                    "'_".to_string()
-                    s
-                }
-                }
+                if s.is_empty() { "'_".to_string() } else { s }
             .collect::<Vec<_>>()
             .collect::<Vec<_>>()
             .join(", ");
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 1178:
 
                     fn lifetime_display(lifetime: Region<'_>) -> String {
                         let s = lifetime.to_string();
-                        if s.is_empty() {
-                            "'_".to_string()
-                            s
-                        }
-                        }
+                        if s.is_empty() { "'_".to_string() } else { s }
                     }
                     // At one point we'd like to elide all lifetimes here, they are irrelevant for
                     // all diagnostics that use this output
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/different_lifetimes.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/find_anon_type.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/util.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs" "/checkout/compiler/rustc_infer/src/infer/free_regions.rs" "/checkout/compiler/rustc_infer/src/traits/engine.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
