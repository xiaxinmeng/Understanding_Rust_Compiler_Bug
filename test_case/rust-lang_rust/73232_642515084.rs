plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 65'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d9273aa5-50bd-496f-bf80-3fef742dc38d.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73232/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73232/merge:refs/remotes/pull/73232/merge
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
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error: expected expression, found `$`
   --> src/bootstrap/tool.rs:610:35
    |
608 |           impl Step for $name {
    |                               - while parsing this item list starting here
609 |               type Output = Option<PathBuf>;
610 |               const DEFAULT: bool = $stable; // FIXME What does this do?
...
653 |           }
    |           - the item list ends here
...
...
660 | / tool_extended!((self, builder),
661 | |     Cargofmt, rustfmt, "src/tools/rustfmt", "cargo-fmt", stable=true, {};
662 | |     CargoClippy, clippy, "src/tools/clippy", "cargo-clippy", stable=true, {};
663 | |     Clippy, clippy, "src/tools/clippy", "clippy-driver", stable=true, {};
...   |
674 | |     Rustfmt, rustfmt, "src/tools/rustfmt", "rustfmt", stable=true, {};
675 | | );
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0046]: not all trait items implemented, missing: `run`, `should_run`
    |
608 |           impl Step for $name {
608 |           impl Step for $name {
    |           ^^^^^^^^^^^^^^^^^^^ missing `run`, `should_run` in implementation
...
660 | / tool_extended!((self, builder),
661 | |     Cargofmt, rustfmt, "src/tools/rustfmt", "cargo-fmt", stable=true, {};
662 | |     CargoClippy, clippy, "src/tools/clippy", "cargo-clippy", stable=true, {};
663 | |     Clippy, clippy, "src/tools/clippy", "clippy-driver", stable=true, {};
...   |
674 | |     Rustfmt, rustfmt, "src/tools/rustfmt", "rustfmt", stable=true, {};
675 | | );
    | 
   ::: src/bootstrap/builder.rs:62:5
    |
    |
62  |       fn run(self, builder: &Builder<'_>) -> Self::Output;
    |       ---------------------------------------------------- `run` from trait
...
68  |       fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_>;
    |       --------------------------------------------------- `should_run` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

---
  local time: Thu Jun 11 09:02:15 UTC 2020
  network time: Thu, 11 Jun 2020 09:02:15 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73232/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73232/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3852) (python)
##[section]Finishing: Finalize Job
