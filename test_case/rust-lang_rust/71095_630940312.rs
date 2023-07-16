plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 8'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200512.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200512.2/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8631931e-58b3-498a-8df1-eb5a19efaab8.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71095/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71095/merge:refs/remotes/pull/71095/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
...................................................................i................................ 1800/10189
.................................................................................................... 1900/10189
......................................................................................i..i.......... 2000/10189
.................................................................................................... 2100/10189
............................................................................iiiii................... 2200/10189
.................................................................................................... 2400/10189
.................................................................................................... 2500/10189
.................................................................................................... 2600/10189
.................................................................................................... 2700/10189
---
........i........................................................................................... 5200/10189
.................................................................................................... 5300/10189
.......................................i............................................................ 5400/10189
................................i................................................................... 5500/10189
.........................................ii.ii........i...i......................................... 5600/10189
...........................................................................................i........ 5800/10189
.................................................................................................... 5900/10189
......................................ii.....................................i...................... 6000/10189
.................................................................................................... 6100/10189
.................................................................................................... 6100/10189
.................................................................................................... 6200/10189
...................................................................................................i 6300/10189
i...i..ii...........i............................................................................... 6400/10189
.................................................................................................... 6600/10189
.................................................................................................... 6700/10189
.................................................................................................... 6700/10189
................................i..ii............................................................... 6800/10189
.................................................................................................... 7000/10189
......................................................................................i............. 7100/10189
.................................................................................................... 7200/10189
.................................................................................................... 7300/10189
---
.................................................................................................... 8100/10189
.................................................................................................... 8200/10189
.................................................................................................... 8300/10189
..........i......................................................................................... 8400/10189
...............................................................iiiiii.iiiiii.i...................... 8500/10189
..................i................................................................................. 8700/10189
.................................................................................................... 8800/10189
.................................................................................................... 8900/10189
.................................................................................................... 9000/10189
---

---- [ui] ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs stdout ----
diff of stderr:

18              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<&str>>
19              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::borrow::Cow<'a, str>>>
20              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::string::String>>
+            and 22 others
22    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
22    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
23    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/alloc-types-no-impls-length-33.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/array-impls/alloc-types-no-impls-length-33.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:6:29
   |
LL |     let v: Vec<_> = [0; 33].into();
   |                             ^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[{integer}; 33]`
   |
   = note: required because of the requirements on the impl of `std::convert::From<[{integer}; 33]>` for `std::vec::Vec<{integer}>`
   = note: required because of the requirements on the impl of `std::convert::Into<std::vec::Vec<{integer}>>` for `[{integer}; 33]`

error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<std::boxed::Box<[i32]>>` is not satisfied
   |
   |
LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <std::boxed::Box<(dyn std::error::Error + 'a)> as std::convert::From<E>>
             <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<&str>>
             <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::borrow::Cow<'a, str>>>
             <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::string::String>>
   = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
   = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
   = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`
error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:15:42
   |
   |
LL |     let boxed_slice = <Box<[i32]>>::from([0; 33]);
   |                                          |
   |                                          |
   |                                          expected an implementor of trait `std::convert::From<[{integer}; 33]>`
   |
   |
   = note: the trait bound `[i32; 33]: std::convert::From<[{integer}; 33]>` is not satisfied
   = note: required because of the requirements on the impl of `std::convert::From<[i32; 33]>` for `std::boxed::Box<[i32]>`


error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::TryFrom<std::boxed::Box<[i32]>>` is not satisfied
   |
   |
LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <std::boxed::Box<[T; N]> as std::convert::TryFrom<std::boxed::Box<[T]>>>

error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::From<std::rc::Rc<[i32]>>` is not satisfied
   |
   |
LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <std::rc::Rc<B> as std::convert::From<std::borrow::Cow<'a, B>>>
             <std::rc::Rc<T> as std::convert::From<T>>
             <std::rc::Rc<T> as std::convert::From<std::boxed::Box<T>>>
             <std::rc::Rc<[T]> as std::convert::From<&[T]>>
           and 9 others
   = note: required because of the requirements on the impl of `std::convert::Into<std::rc::Rc<[i32; 33]>>` for `std::rc::Rc<[i32]>`
   = note: required because of the requirements on the impl of `std::convert::TryFrom<std::rc::Rc<[i32]>>` for `std::rc::Rc<[i32; 33]>`

error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::TryFrom<std::rc::Rc<[i32]>>` is not satisfied
   |
   |
LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <std::rc::Rc<[T; N]> as std::convert::TryFrom<std::rc::Rc<[T]>>>

error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::From<std::sync::Arc<[i32]>>` is not satisfied
   |
   |
LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <std::sync::Arc<B> as std::convert::From<std::borrow::Cow<'a, B>>>
             <std::sync::Arc<T> as std::convert::From<T>>
             <std::sync::Arc<T> as std::convert::From<std::boxed::Box<T>>>
             <std::sync::Arc<[T]> as std::convert::From<&[T]>>
           and 9 others
   = note: required because of the requirements on the impl of `std::convert::Into<std::sync::Arc<[i32; 33]>>` for `std::sync::Arc<[i32]>`
   = note: required because of the requirements on the impl of `std::convert::TryFrom<std::sync::Arc<[i32]>>` for `std::sync::Arc<[i32; 33]>`

error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::TryFrom<std::sync::Arc<[i32]>>` is not satisfied
   |
   |
LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <std::sync::Arc<[T; N]> as std::convert::TryFrom<std::sync::Arc<[T]>>>
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0277`.

---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:58:39
Build completed unsuccessfully in 0:58:39
== clock drift check ==
  local time: Tue May 19 16:39:16 UTC 2020
  network time: Tue, 19 May 2020 16:39:16 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71095/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71095/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3571) (python)
##[section]Finishing: Finalize Job
