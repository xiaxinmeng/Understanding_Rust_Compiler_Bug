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
Diff in /checkout/compiler/rustc_mir/src/const_eval/eval_queries.rs at line 365:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/const_eval/eval_queries.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                             "the raw bytes of the constant ({}",
                             display_allocation(
                                 *ecx.tcx,
-                                ecx.tcx.global_alloc(mplace.ptr.assert_ptr().alloc_id).unwrap_memory()
+                                ecx.tcx
+                                    .global_alloc(mplace.ptr.assert_ptr().alloc_id)
+                                    .unwrap_memory()
                         ));
                         ));
                         diag.emit();
Build completed unsuccessfully in 0:00:18
