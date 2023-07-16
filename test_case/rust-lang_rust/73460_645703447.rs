plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 7'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200604.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/25934668-ae0a-4ad5-a8ba-fca20496d4eb.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73460/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73460/merge:refs/remotes/pull/73460/merge
---
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in a297404f768d
Removing intermediate container a297404f768d
 ---> 755c587daf39
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
Removing intermediate container 8a4e6c84911e
 ---> 460b50c8a254
Successfully built 460b50c8a254
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:460b50c8a25485448f839d16d2c136cf0ac76ed1adf72ec75f6bd2a353908a53
Uploading finished image to https://ci-caches.rust-lang.org/docker/d3f0b748e51c65823b26e5c9aa3312fcd51003139be9a43290b3a342a55aa1a19cca0eefe8cd2813f66ed4bd2ebafe54507cc7aa11fb4b3d0cb6de76d4374fee
upload failed: - to s3://rust-lang-ci-sccache2/docker/d3f0b748e51c65823b26e5c9aa3312fcd51003139be9a43290b3a342a55aa1a19cca0eefe8cd2813f66ed4bd2ebafe54507cc7aa11fb4b3d0cb6de76d4374fee An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
[CI_JOB_NAME=mingw-check]
== clock drift check ==
  local time: Wed Jun 17 23:51:47 UTC 2020
  network time: Wed, 17 Jun 2020 23:51:47 GMT
---
    Checking chalk-rust-ir v0.10.0
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
configure: rust.verify-llvm-ir  := True
configure: rust.debug-assertions := True
configure: build.cargo-native-static := True
configure: llvm.assertions      := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
processor : 0
vendor_id : GenuineIntel
cpu family : 6
model  : 85
model name : Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz
microcode : 0xffffffff
cpu MHz  : 2593.904
cache size : 36608 KB
physical id : 0
---
processor : 1
vendor_id : GenuineIntel
cpu family : 6
model  : 85
model name : Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz
microcode : 0xffffffff
cpu MHz  : 2593.904
cache size : 36608 KB
physical id : 0
---
Hugepagesize:       2048 kB
DirectMap4k:      128960 kB
DirectMap2M:     4065280 kB
DirectMap1G:     5242880 kB
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
    Finished dev [unoptimized] target(s) in 0.15s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking chalk-rust-ir v0.10.0
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustc_mir/transform/generator.rs at line 771:
         SourceInfo::outermost(body_span.shrink_to_lo()),
         SourceInfo::outermost(body_span.shrink_to_hi()),
         SourceInfo::outermost(body_span.shrink_to_hi()),
-    ].iter().copied().collect();
+    .iter()
+    .copied()
+    .collect();
 
 
     // Build the generator variant field list.
     // Create a map from local indices to generator struct indices.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/transform/generator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:31
== clock drift check ==
  local time: Thu Jun 18 00:05:56 UTC 2020
  network time: Thu, 18 Jun 2020 00:05:56 GMT
  network time: Thu, 18 Jun 2020 00:05:56 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73460/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73460/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (5417) (python)
##[section]Finishing: Finalize Job
