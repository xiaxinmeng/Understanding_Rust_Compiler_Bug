plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2510c3ee-d9df-4726-b813-59453672ecc5.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71994/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71994/merge:refs/remotes/pull/71994/merge
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
   Compiling cc v1.0.50
    Checking core v0.0.0 (/checkout/src/libcore)
thread 'main' panicked at '

 failed to run "build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "cc" "--edition=2018" "/cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.50/src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=3" "-C" "debuginfo=0" "-C" "metadata=9e7bf81b9fe8db6d" "-C" "extra-filename=-9e7bf81b9fe8db6d" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "--cap-lints" "allow" "-Zbinary-dep-depinfo"', src/bootstrap/bin/rustc.rs:205:54
thread 'main' panicked at '


 failed to run "build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "core" "--edition=2018" "src/libcore/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "opt-level=3" "-C" "debuginfo=0" "-C" "metadata=c9663c65a085fa4c" "-C" "extra-filename=-c9663c65a085fa4c" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "--cfg=bootstrap" "-Zmacro-backtrace" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Dwarnings" "-Cprefer-dynamic" "-Zbinary-dep-depinfo" "--sysroot" "build/x86_64-unknown-linux-gnu/stage0-sysroot" "-C" "debug-assertions=n" "-Z" "force-unstable-if-unmarked"', src/bootstrap/bin/rustc.rs:205:54
error: could not compile `core`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
---
  local time: Thu May  7 23:02:01 UTC 2020
  network time: Thu, 07 May 2020 23:02:01 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71994/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71994/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4180) (python)
##[section]Finishing: Finalize Job
