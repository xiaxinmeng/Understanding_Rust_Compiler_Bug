plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 8'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9d05b806-bb2c-46ba-a83a-0f2bc72a3d49.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73464/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73464/merge:refs/remotes/pull/73464/merge
---
 ---> 7c035a44a114
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 6a7a9b8f9bf0
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 56a540647f55
Successfully built 56a540647f55
Successfully tagged rust-ci:latest
Built container sha256:56a540647f55964f5c32c38aa194ec9408e923d4c62f7244b7ac91f9179aff2a
---
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking chalk-solve v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
configure: llvm.ccache          := sccache
configure: rust.channel         := nightly
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
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
cpu MHz  : 2593.906
cache size : 36608 KB
physical id : 0
---
processor : 1
vendor_id : GenuineIntel
cpu family : 6
model  : 85
model name : Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz
microcode : 0xffffffff
cpu MHz  : 2593.906
cache size : 36608 KB
physical id : 0
---
Hugepagesize:       2048 kB
DirectMap4k:      116672 kB
DirectMap2M:     4077568 kB
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
Build completed successfully in 0:00:24
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.16s
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
Found 0 error codes with no tests
Done!


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
Build completed unsuccessfully in 0:00:27
Build completed unsuccessfully in 0:00:27
== clock drift check ==
  local time: Thu Jun 18 02:33:53 UTC 2020
  network time: Thu, 18 Jun 2020 02:33:53 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73464/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73464/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3717) (python)
##[section]Finishing: Finalize Job
