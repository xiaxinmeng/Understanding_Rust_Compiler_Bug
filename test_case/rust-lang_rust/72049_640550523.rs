plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 13'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/65121d43-8480-49e1-bc4b-97ab6c47f331.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72049/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72049/merge:refs/remotes/pull/72049/merge
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
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
error: unknown start of token: `
    --> src/librustc_session/session.rs:1306:19
     |
1306 |                   `-C prefer-dynamic` when targeting MSVC",
1306 |                   `-C prefer-dynamic` when targeting MSVC",
     |                   ^
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
1306 |                   '-C prefer-dynamic` when targeting MSVC",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1306:37
     |
     |
1306 |                   `-C prefer-dynamic` when targeting MSVC",
     |                                     ^
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
1306 |                   `-C prefer-dynamic' when targeting MSVC",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1315:23
     |
     |
1315 |                 "File `{}` passed to `-C profile-use` does not exist.",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1315 |                 "File '{}` passed to `-C profile-use` does not exist.",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1315:26
     |
     |
1315 |                 "File `{}` passed to `-C profile-use` does not exist.",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1315 |                 "File `{}' passed to `-C profile-use` does not exist.",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1315:38
     |
     |
1315 |                 "File `{}` passed to `-C profile-use` does not exist.",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1315 |                 "File `{}` passed to '-C profile-use` does not exist.",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1315:53
     |
     |
1315 |                 "File `{}` passed to `-C profile-use` does not exist.",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1315 |                 "File `{}` passed to `-C profile-use' does not exist.",

error: unknown start of token: \
    --> src/librustc_session/session.rs:1325:79
     |
     |
1325 |                 "panic=unwind requires unwind tables, they cannot be disabled \
     |                                                                               ^

error: unknown start of token: `
    --> src/librustc_session/session.rs:1326:27
     |
1326 |                      with `-C force-unwind-tables=no`.",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1326 |                      with '-C force-unwind-tables=no`.",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1326:53
     |
     |
1326 |                      with `-C force-unwind-tables=no`.",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1326 |                      with `-C force-unwind-tables=no'.",

error: unknown start of token: \
    --> src/librustc_session/session.rs:1332:78
     |
     |
1332 |                 "target requires unwind tables, they cannot be disabled with \
     |                                                                              ^

error: unknown start of token: `
    --> src/librustc_session/session.rs:1333:22
     |
1333 |                      `-C force-unwind-tables=no`.",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1333 |                      '-C force-unwind-tables=no`.",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1333:48
     |
     |
1333 |                      `-C force-unwind-tables=no`.",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1333 |                      `-C force-unwind-tables=no'.",

error: unknown start of token: \
    --> src/librustc_session/session.rs:1350:75
     |
     |
1350 |             "Profile-guided optimization does not yet work in conjunction \
     |                                                                           ^

error: unknown start of token: `
    --> src/librustc_session/session.rs:1351:24
     |
1351 |                   with `-Cpanic=unwind` on Windows when targeting MSVC. \
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1351 |                   with '-Cpanic=unwind` on Windows when targeting MSVC. \

error: unknown start of token: `
    --> src/librustc_session/session.rs:1351:39
     |
     |
1351 |                   with `-Cpanic=unwind` on Windows when targeting MSVC. \
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1351 |                   with `-Cpanic=unwind' on Windows when targeting MSVC. \

error: unknown start of token: \
    --> src/librustc_session/session.rs:1351:73
     |
     |
1351 |                   with `-Cpanic=unwind` on Windows when targeting MSVC. \

error: unknown start of token: `
    --> src/librustc_session/session.rs:1380:52
     |
     |
1380 |                 "{:?}Sanitizer only works with the `{}` target",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1380 |                 "{:?}Sanitizer only works with the '{}` target",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1380:55
     |
     |
1380 |                 "{:?}Sanitizer only works with the `{}` target",
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1380 |                 "{:?}Sanitizer only works with the `{}' target",

error: unknown start of token: `
    --> src/librustc_session/session.rs:1382:41
     |
     |
1382 |                 supported_targets.join("` or `")
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1382 |                 supported_targets.join("' or `")

error: unknown start of token: `
    --> src/librustc_session/session.rs:1382:46
     |
     |
1382 |                 supported_targets.join("` or `")
     |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
     |
     |
1382 |                 supported_targets.join("` or '")

error: unterminated double quote string
    --> src/librustc_session/session.rs:1382:47
     |
     |
1382 |                   supported_targets.join("` or `")
1383 | |             ));
1384 | |         }
1385 | |     }
...    |
...    |
1434 | |
1435 | | pub type CompileResult = Result<(), ErrorReported>;

error: aborting due to 21 previous errors

error: could not compile `rustc_session`.
error: could not compile `rustc_session`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:48
== clock drift check ==
  local time: Mon Jun  8 11:37:00 UTC 2020
  local time: Mon Jun  8 11:37:00 UTC 2020
  network time: Mon, 08 Jun 2020 11:37:00 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72049/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72049/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4475) (python)
##[section]Finishing: Finalize Job
