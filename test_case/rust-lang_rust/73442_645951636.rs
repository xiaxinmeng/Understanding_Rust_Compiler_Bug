plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 5'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/39bf4f3d-7436-414a-bd27-d4cbe01ef712.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73442/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73442/merge:refs/remotes/pull/73442/merge
---
 ---> 31fea614d2f3
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> 4195cadf126d
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 4e90f6b48f05
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> dfa0a356d899
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.................................................................................................... 1900/10320
.................................................................................................... 2000/10320
...............i..i................................................................................. 2100/10320
.................................................................................................... 2200/10320
.....iiiii.......................................................................................... 2300/10320
.................................................................................................... 2500/10320
.................................................................................................... 2600/10320
.................................................................................................... 2700/10320
.................................................................................................... 2800/10320
---
.................................................................................................... 6000/10320
.......ii.....................................i..................................................... 6100/10320
.................................................................................................... 6200/10320
.................................................................................................... 6300/10320
......................................................................ii...i..ii...........i........ 6400/10320
.................................................................................................... 6600/10320
.................................................................................................... 6700/10320
.................................................................................................... 6800/10320
.................................................................................................... 6800/10320
....i..ii........................................................................................... 6900/10320
.................................................................................................... 7100/10320
...........................................................i........................................ 7200/10320
.................................................................................................... 7300/10320
.................................................................................................... 7400/10320
---
.................................................................................................... 8200/10320
.................................................................................................... 8300/10320
.................................................................................................... 8400/10320
.i.................................................................................................. 8500/10320
.......................................................iiiiii.iiiiii.i.............................. 8600/10320
............i....................................................................................... 8800/10320
.................................................................................................... 8900/10320
.................................................................................................... 9000/10320
.................................................................................................... 9100/10320
---
---- [mir-opt] mir-opt/issue-72181.rs stdout ----
1 // MIR for `main` 0 mir_map
2 
3 | User Type Annotations
- | 0: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6169 ~ core[2a38]::fmt[0]::{{impl}}[2]::new_v1[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0))], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6167 ~ core[2a38]::fmt[0]::{{impl}}[2]), self_ty: std::fmt::Arguments<'_> }) }) } at $SRC_DIR/libstd/macros.rs:LL:COL
- | 1: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6162 ~ core[2a38]::fmt[0]::{{impl}}[1]::new[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0)), ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6160 ~ core[2a38]::fmt[0]::{{impl}}[1]), self_ty: std::fmt::ArgumentV1<'_> }) }) } at $SRC_DIR/libstd/macros.rs:LL:COL
- | 2: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6169 ~ core[2a38]::fmt[0]::{{impl}}[2]::new_v1[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0))], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6167 ~ core[2a38]::fmt[0]::{{impl}}[2]), self_ty: std::fmt::Arguments<'_> }) }) } at $SRC_DIR/libstd/macros.rs:LL:COL
- | 3: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6162 ~ core[2a38]::fmt[0]::{{impl}}[1]::new[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0)), ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6160 ~ core[2a38]::fmt[0]::{{impl}}[1]), self_ty: std::fmt::ArgumentV1<'_> }) }) } at $SRC_DIR/libstd/macros.rs:LL:COL
+ | 0: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6169 ~ core[a39f]::fmt[0]::{{impl}}[2]::new_v1[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0))], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6167 ~ core[a39f]::fmt[0]::{{impl}}[2]), self_ty: std::fmt::Arguments<'_> }) }) } at $SRC_DIR/libstd/macros.rs:LL:COL
+ | 1: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6162 ~ core[a39f]::fmt[0]::{{impl}}[1]::new[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0)), ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6160 ~ core[a39f]::fmt[0]::{{impl}}[1]), self_ty: std::fmt::ArgumentV1<'_> }) }) } at $SRC_DIR/libstd/macros.rs:LL:COL
+ | 2: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6169 ~ core[a39f]::fmt[0]::{{impl}}[2]::new_v1[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0))], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6167 ~ core[a39f]::fmt[0]::{{impl}}[2]), self_ty: std::fmt::Arguments<'_> }) }) } at $SRC_DIR/libstd/macros.rs:LL:COL
+ | 3: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6162 ~ core[a39f]::fmt[0]::{{impl}}[1]::new[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0)), ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6160 ~ core[a39f]::fmt[0]::{{impl}}[1]), self_ty: std::fmt::ArgumentV1<'_> }) }) } at $SRC_DIR/libstd/macros.rs:LL:COL
9 fn main() -> () {
10     let mut _0: ();                      // return place in scope 0 at $DIR/issue-72181.rs:21:11: 21:11


thread '[mir-opt] mir-opt/issue-72181.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue-72181/rustc.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3194:25


failures:
    [mir-opt] mir-opt/issue-72181.rs
    [mir-opt] mir-opt/issue-72181.rs

test result: FAILED. 107 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:06:39
Build completed unsuccessfully in 1:06:39
== clock drift check ==
  local time: Thu Jun 18 11:16:45 UTC 2020
  network time: Thu, 18 Jun 2020 11:16:46 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73442/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73442/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3560) (python)
##[section]Finishing: Finalize Job
