plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 6'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200614.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200614.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e124395d-84b5-4522-a073-acd7cd4006dc.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73562/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73562/merge:refs/remotes/pull/73562/merge
---
 ---> a9ec21d337b3
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 5ff2c13d8dba
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 6b931e755c7e
Successfully built 6b931e755c7e
Successfully tagged rust-ci:latest
Built container sha256:6b931e755c7ea1f69816640eb9df74fafd40a545d0e5aa8341d35009dabb0f3c
---
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking chalk-solve v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
configure: rust.verify-llvm-ir  := True
configure: dist.missing-tools   := True
configure: build.cargo-native-static := True
configure: rust.channel         := nightly
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      149440 kB
DirectMap2M:     5093376 kB
DirectMap1G:     4194304 kB
+ python3 ../x.py test src/tools/expand-yaml-anchors
Ensuring the YAML anchors in the GitHub Actions config were expanded
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
---
   Compiling linked-hash-map v0.5.2
   Compiling lazy_static v1.4.0
   Compiling yaml-rust v0.4.3
   Compiling quote v1.0.2
   Compiling thiserror-impl v1.0.5
   Compiling thiserror v1.0.5
   Compiling yaml-merge-keys v0.4.0
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
Build completed successfully in 0:00:33
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.21s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking chalk-solve v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
   Compiling cargo_metadata v0.9.1
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 27.48s
tidy check
tidy error: /checkout/src/librustc_error_codes/error_codes/E0432.md:47: line longer than 80 chars
Found 503 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
Build completed unsuccessfully in 0:00:38
Build completed unsuccessfully in 0:00:38
== clock drift check ==
  local time: Sat Jun 20 22:18:09 UTC 2020
  network time: Sat, 20 Jun 2020 22:18:09 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73562/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73562/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3621) (python)
##[section]Finishing: Finalize Job
