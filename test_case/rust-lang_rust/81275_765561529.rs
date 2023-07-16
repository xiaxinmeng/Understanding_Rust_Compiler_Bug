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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_interface/src/queries.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/compiler/rustc_interface/src/queries.rs at line 421:
             // We assume that no queries are run past here. If there are new queries
             // after this point, they'll show up as "<unknown>" in self-profiling data.
             {
-                let _prof_timer = queries.session().prof.generic_activity("self_profile_alloc_query_strings");
+                let _prof_timer =
+                    queries.session().prof.generic_activity("self_profile_alloc_query_strings");
                 gcx.peek_mut().enter(|tcx| tcx.alloc_self_profile_query_strings());
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:22
