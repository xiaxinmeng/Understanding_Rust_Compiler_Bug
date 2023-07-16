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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 1490:
                             *sp,
                             format!(
                                 "{}{}{} {}{}",
-                                if sp.is_desugaring(DesugaringKind::Async) && !returned_async_output_error {
+                                if sp.is_desugaring(DesugaringKind::Async)
+                                    && !returned_async_output_error
+                                {
                                     "checked the `Output` of this `async fn`, "
                                 } else if count == 1 {
                                     "the "
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 1503:
                                 pluralize!(count),
                         );
                         );
-                        if sp.is_desugaring(DesugaringKind::Async) && returned_async_output_error == false {
+                        if sp.is_desugaring(DesugaringKind::Async)
+                            && returned_async_output_error == false
+                        {
                             err.note("while checking the return type of the `async fn`");
                             returned_async_output_error = true;
                         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/compiler/rustc_infer/src/infer/lub.rs" "/checkout/compiler/rustc_infer/src/infer/mod.rs" "/checkout/compiler/rustc_infer/src/infer/combine.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/util.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/named_anon_conflict.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
