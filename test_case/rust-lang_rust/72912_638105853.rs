plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 69'
Agent machine name: 'fv-az578'
Current agent version: '2.169.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f2689815-5f47-4f0c-9039-d5a9971c6bd5.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72912/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72912/merge:refs/remotes/pull/72912/merge
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
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: rust.channel         := nightly
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      153536 kB
DirectMap2M:     2992128 kB
DirectMap1G:     6291456 kB
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
Build completed successfully in 0:00:23
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.16s
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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustc_parse/lexer/mod.rs at line 191:
                         "unterminated block comment"
                     };
                     let last_bpos = self.pos;
-                    self.sess.span_diagnostic.struct_span_fatal_with_code(
-                        self.mk_sp(start, last_bpos),
-                        msg,
-                        error_code!(E0758),
-                    ).emit();
+                        .span_diagnostic
+                        .struct_span_fatal_with_code(
+                        .struct_span_fatal_with_code(
+                            self.mk_sp(start, last_bpos),
+                            msg,
+                            error_code!(E0758),
+                        .emit();
+                        .emit();
                     FatalError.raise();
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_parse/lexer/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:32
== clock drift check ==
  local time: Wed Jun  3 09:28:57 UTC 2020
  network time: Wed, 03 Jun 2020 09:28:57 GMT
  network time: Wed, 03 Jun 2020 09:28:57 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72912/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72912/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3501) (python)
##[section]Finishing: Finalize Job
