plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200512.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200512.2/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0309cbd5-6fd9-42b3-b7b7-882aa2d67dc9.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71349/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71349/merge:refs/remotes/pull/71349/merge
---
 ---> 3adb0605cc65
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 28dbc326cb7f
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 537a01811900
Successfully built 537a01811900
Successfully tagged rust-ci:latest
Built container sha256:537a018119009dc218456238dec90b5530050db1e2a1e166550c218003f6159d
---
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
configure: dist.missing-tools   := True
configure: llvm.ccache          := sccache
configure: build.cargo-native-static := True
configure: rust.dist-src        := False
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      139200 kB
DirectMap2M:     5103616 kB
DirectMap1G:     4194304 kB
+ python3 ../x.py test src/tools/expand-yaml-anchors
Ensuring the YAML anchors in the GitHub Actions config were expanded
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.11
   Compiling linked-hash-map v0.5.2
   Compiling lazy_static v1.4.0
   Compiling lazy_static v1.4.0
   Compiling yaml-rust v0.4.3
   Compiling quote v1.0.2
   Compiling thiserror-impl v1.0.5
   Compiling thiserror v1.0.5
   Compiling yaml-merge-keys v0.4.0
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
Build completed successfully in 0:00:27
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.20s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustc_mir/monomorphize/partitioning.rs at line 155:
     // local functions the definition of which is marked with `#[inline]`.
     let mut post_inlining = {
         let _prof_timer = tcx.prof.generic_activity("cgu_partitioning_place_inline_items");
-        let is_debug_incremental = tcx.sess.opts.optimize == OptLevel::No && tcx.sess.opts.incremental.is_some();
+        let is_debug_incremental =
+            tcx.sess.opts.optimize == OptLevel::No && tcx.sess.opts.incremental.is_some();
         place_inlined_mono_items(initial_partitioning, inlining_map, is_debug_incremental)
 
Diff in /checkout/src/librustc_mir/monomorphize/partitioning.rs at line 228:
             InstantiationMode::LocalCopy => continue,
         }
         }
 
-        let characteristic_def_id = characteristic_def_id_of_mono_item(tcx, mono_item, is_debug_incremental);
+        let characteristic_def_id =
+            characteristic_def_id_of_mono_item(tcx, mono_item, is_debug_incremental);
         let is_volatile = is_incremental_build && mono_item.is_generic_fn();
 
         let codegen_unit_name = match (characteristic_def_id, mono_item.is_local()) {
Diff in /checkout/src/librustc_mir/monomorphize/partitioning.rs at line 235:
             (Some(def_id), false) if is_debug_incremental => {
                 let crate_name = tcx.crate_name(def_id.krate);
-                cgu_name_builder.build_cgu_name(LOCAL_CRATE, &[&*crate_name.as_str(), if mono_item.has_closure_generic_argument() { "has_closure" } else { "" }], Some("cgu"))
-            },
+                cgu_name_builder.build_cgu_name(
+                    &[
+                    &[
+                        &*crate_name.as_str(),
+                        if mono_item.has_closure_generic_argument() { "has_closure" } else { "" },
+                    ],
+                    Some("cgu"),
+            }
             (Some(def_id), _) => compute_codegen_unit_name(
                 tcx,
                 tcx,
                 cgu_name_builder,
Diff in /checkout/src/librustc_mir/monomorphize/partitioning.rs at line 468:
     assert!(target_cgu_count >= 1);
     let codegen_units = &mut initial_partitioning.codegen_units;
 
-    if tcx.is_compiler_builtins(LOCAL_CRATE) || (tcx.dep_graph.is_fully_enabled() && tcx.sess.opts.optimize == OptLevel::No) {
+    if tcx.is_compiler_builtins(LOCAL_CRATE)
+        || (tcx.dep_graph.is_fully_enabled() && tcx.sess.opts.optimize == OptLevel::No)
+    {
         // Compiler builtins require some degree of control over how mono items
         // are partitioned into compilation units. Provide it by keeping the
         // original partitioning when compiling the compiler builtins crate.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/monomorphize/partitioning.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:36
== clock drift check ==
  local time: Wed May 20 13:32:30 UTC 2020
  network time: Wed, 20 May 2020 13:32:30 GMT
  network time: Wed, 20 May 2020 13:32:30 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71349/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71349/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3591) (python)
##[section]Finishing: Finalize Job
