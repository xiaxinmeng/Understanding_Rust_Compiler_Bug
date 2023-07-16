plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c75f797f-cccc-4137-be6b-cf207e071cd2.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71991/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71991/merge:refs/remotes/pull/71991/merge
---
 ---> f883e675ad62
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> c0b156eb069c
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 8541bab6b38c
Successfully built 8541bab6b38c
Successfully tagged rust-ci:latest
Built container sha256:8541bab6b38c07f1b7eb787539b9cbe93daa6ac4458d3d7bd8a8921622a14ba1
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
    Checking serde v1.0.99
    Checking serde_json v1.0.40
    Checking semver v0.9.0
    Checking url v2.1.0
error[E0658]: use of unstable library feature 'iter_starts_with': recently added
    |
    |
361 |             if input.starts_with('#') {
    |
    |
    = help: add `#![feature(iter_starts_with)]` to the crate attributes to enable
error[E0277]: `char` is not an iterator
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/url-2.1.0/src/parser.rs:361:34
    |
    |
361 |             if input.starts_with('#') {
    |                                  ^^^ `char` is not an iterator
    = help: the trait `std::iter::Iterator` is not implemented for `char`
    = note: required because of the requirements on the impl of `std::iter::IntoIterator` for `char`


error[E0658]: use of unstable library feature 'iter_starts_with': recently added
    |
    |
379 |         if input.is_empty() || !input.starts_with(ascii_alpha) {
    |
    |
    = help: add `#![feature(iter_starts_with)]` to the crate attributes to enable

error[E0277]: `fn(char) -> bool {parser::ascii_alpha}` is not an iterator
    |
    |
379 |         if input.is_empty() || !input.starts_with(ascii_alpha) {
    |                                                   ^^^^^^^^^^^ `fn(char) -> bool {parser::ascii_alpha}` is not an iterator
    |
    = help: the trait `std::iter::Iterator` is not implemented for `fn(char) -> bool {parser::ascii_alpha}`
    = note: required because of the requirements on the impl of `std::iter::IntoIterator` for `fn(char) -> bool {parser::ascii_alpha}`

error[E0658]: use of unstable library feature 'iter_starts_with': recently added
    |
    |
411 |                 self.log_violation_if(ExpectedFileDoubleSlash, || !input.starts_with("//"));
    |
    |
    = help: add `#![feature(iter_starts_with)]` to the crate attributes to enable
error[E0277]: `&str` is not an iterator
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/url-2.1.0/src/parser.rs:411:86
    |
    |
411 |                 self.log_violation_if(ExpectedFileDoubleSlash, || !input.starts_with("//"));
    |                                                                                      ^^^^ `&str` is not an iterator; try calling `.chars()` or `.bytes()`
    = help: the trait `std::iter::Iterator` is not implemented for `&str`
    = note: required because of the requirements on the impl of `std::iter::IntoIterator` for `&str`

error: aborting due to 6 previous errors
---
  local time: Tue Jun 16 23:00:16 UTC 2020
  network time: Tue, 16 Jun 2020 23:00:16 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71991/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71991/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3382) (python)
##[section]Finishing: Finalize Job
