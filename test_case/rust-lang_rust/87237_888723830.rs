plain
Found online and idle hosted runner in the current repository's organization account that matches the required labels: 'ubuntu-latest-xl'
Waiting for a Hosted runner in the 'organization' to pick this job...
Job is waiting for a hosted runner to come online.
Job is about to start running on the hosted runner: Hosted Agent (hosted)
##[group]Operating System
Ubuntu
18.04.5
LTS
---
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
* highest error code: E0783
Found 499 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_feature/src/active.rs:681: feature const_for is not sorted by "since" (version number)
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
