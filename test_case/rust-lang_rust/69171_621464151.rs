plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/64e61e0a-9a35-4eed-a76f-b8025dc08495.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69171/merge:refs/remotes/pull/69171/merge
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
Looks like docker image is the same as before, not uploading
[CI_JOB_NAME=x86_64-gnu-llvm-8]
[CI_JOB_NAME=x86_64-gnu-llvm-8]
== clock drift check ==
  local time: Wed Apr 29 20:01:49 UTC 2020
  network time: Wed, 29 Apr 2020 20:01:49 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.dist-src        := False
---
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 1m 03s
{"reason":"build-finished","success":true}
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cfg-if v0.1.10
   Compiling libc v0.2.69
   Compiling semver-parser v0.7.0
---
   Compiling rustc_span v0.0.0 (/checkout/src/librustc_span)
   Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
   Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
   Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
   Compiling rustc-main v0.0.0 (/checkout/src/rustc)
    Finished release [optimized] target(s) in 22m 07s
{"reason":"build-finished","success":true}
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.50
   Compiling core v0.0.0 (/checkout/src/libcore)
---
   Compiling term v0.0.0 (/checkout/src/libterm)
   Compiling unicode-width v0.1.6
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/src/libtest)
    Finished {"reason":"build-finished","success":true}
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cfg-if v0.1.10
   Compiling libc v0.2.69
---
   Compiling rustc_span v0.0.0 (/checkout/src/librustc_span)
   Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
   Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
   Compiling rustc-main v0.0.0 (/checkout/src/rustc)
    Finished release [optimized] target(s) in 23m 22s
{"reason":"build-finished","success":true}
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building test helpers
---
   Compiling serde_derive v1.0.81
   Compiling serde_json v1.0.40
   Compiling rustfix v0.5.0
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
{"reason":"build-finished","success":true}
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9964 tests
.................................................................................................... 100/9964
---
..................i................................................................................. 1800/9964
.................................................................................................... 1900/9964
.................................i.................................................................. 2000/9964
.................................................................................................... 2100/9964
.......................iiiii........................................................................ 2200/9964
.................................................................................................... 2400/9964
.................................................................................................... 2500/9964
.................................................................................................... 2600/9964
.................................................................................................... 2700/9964
---
.....i...............i.............................................................................. 5100/9964
.................................................................................................... 5200/9964
...................................................i................................................ 5300/9964
..........................................i......................................................... 5400/9964
...........................................ii.ii........i...i....................................... 5500/9964
..........................................................................................i......... 5700/9964
.................................................................................................... 5800/9964
............................ii.....................................i................................ 5900/9964
.................................................................................................... 6000/9964
.................................................................................................... 6000/9964
.................................................................................................... 6100/9964
..............................................................ii...i..ii...........i................ 6200/9964
.................................................................................................... 6400/9964
.................................................................................................... 6500/9964
.................................................................................................... 6500/9964
............................................................................................i..ii... 6600/9964
.................................................................................................... 6800/9964
.............................................................................................i...... 6900/9964
.................................................................................................... 7000/9964
.................................................................................................... 7100/9964
---
.................................................................................................... 7900/9964
.................................................................................................... 8000/9964
.................................................................................................... 8100/9964
...i................................................................................................ 8200/9964
....................................................iiiiii.iiiii.i.................................. 8300/9964
.....i.............................................................................................. 8500/9964
.................................................................................................... 8600/9964
.................................................................................................... 8700/9964
.................................................................................................... 8800/9964
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 187 tests
iiii......i..............ii.i..........i.............................i..i..................i....i... 100/187
.........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 5.675
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 20 tests
.F.FFFFFFFiiiiiiiiiF

---- [assembly] assembly/asm/aarch64-types.rs stdout ----

error: compilation failed!
error: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/aarch64-types.rs" "-Zthreads=1" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid operand in inline asm: 'bl ${0:c}'

error: <inline asm>:1:2: error: too few operands for instruction
        bl 



error: invalid operand in inline asm: 'adr x0, ${0:c}'

error: <inline asm>:1:10: error: unknown token in expression
        adr x0, 


error: aborting due to 4 previous errors



------------------------------------------


---- [assembly] assembly/asm/riscv-modifiers.rs stdout ----
error: compilation failed!
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/riscv-modifiers.rs" "-Zthreads=1" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/riscv-modifiers/riscv-modifiers.s" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target" "riscv64gc-unknown-linux-gnu" "-C" "target-feature=+f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/riscv-modifiers/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Could not create LLVM TargetMachine for triple: riscv64-unknown-linux-gnu: No available targets are compatible with triple "riscv64-unknown-linux-gnu"

------------------------------------------



---- [assembly] assembly/asm/riscv-types.rs#riscv32 stdout ----

error in revision `riscv32`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/riscv-types.rs" "-Zthreads=1" "--cfg" "riscv32" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/riscv-types.riscv32/riscv-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "riscv32imac-unknown-none-elf" "-C" "target-feature=+d" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/riscv-types.riscv32/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Could not create LLVM TargetMachine for triple: riscv32: No available targets are compatible with triple "riscv32"

------------------------------------------



---- [assembly] assembly/asm/riscv-types.rs#riscv64 stdout ----

error in revision `riscv64`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/riscv-types.rs" "-Zthreads=1" "--cfg" "riscv64" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/riscv-types.riscv64/riscv-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "riscv64imac-unknown-none-elf" "-C" "target-feature=+d" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/riscv-types.riscv64/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Could not create LLVM TargetMachine for triple: riscv64: No available targets are compatible with triple "riscv64"

------------------------------------------



---- [assembly] assembly/asm/arm-types.rs stdout ----

error: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/arm-types.rs" "-Zthreads=1" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/arm-types/arm-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "armv7-unknown-linux-gnueabihf" "-C" "target-feature=+neon" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/arm-types/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid operand in inline asm: 'bl ${0:c}'

error: <inline asm>:1:2: error: invalid instruction, any one of the following would fix this:
        bl 



error: <inline asm>:1:4: note: too few operands for instruction
        bl 



error: <inline asm>:1:4: note: invalid operand for instruction
        bl 



error: invalid operand in inline asm: 'adr r0, ${0:c}'

error: <inline asm>:1:10: error: unexpected token in operand
        adr r0, 


error: couldn't allocate output register for constraint 'x'
   --> /checkout/src/test/assembly/asm/arm-types.rs:191:1
   --> /checkout/src/test/assembly/asm/arm-types.rs:191:1
    |
191 | check!(sreg_low16_i32 i32 sreg_low16 "vmov.f32");

error: aborting due to 7 previous errors



------------------------------------------


---- [assembly] assembly/asm/x86-modifiers.rs#i686 stdout ----
error in revision `i686`: compilation failed!
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/x86-modifiers.rs" "-Zthreads=1" "--cfg" "i686" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-modifiers.i686/x86-modifiers.s" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target" "i686-unknown-linux-gnu" "-C" "llvm-args=--x86-asm-syntax=intel" "-C" "target-feature=+avx512bw" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-modifiers.i686/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
LLVM ERROR: Bad $ operand number in inline asm string: 'mov ${0:k}, ${0:k}'
------------------------------------------



---- [assembly] assembly/asm/x86-modifiers.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/x86-modifiers.rs" "-Zthreads=1" "--cfg" "x86_64" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-modifiers.x86_64/x86-modifiers.s" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target" "x86_64-unknown-linux-gnu" "-C" "llvm-args=--x86-asm-syntax=intel" "-C" "target-feature=+avx512bw" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-modifiers.x86_64/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
LLVM ERROR: Bad $ operand number in inline asm string: 'mov ${0:q}, ${0:q}'
------------------------------------------



---- [assembly] assembly/asm/x86-types.rs#i686 stdout ----
error in revision `i686`: compilation failed!
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/x86-types.rs" "-Zthreads=1" "--cfg" "i686" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-types.i686/x86-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "i686-unknown-linux-gnu" "-C" "llvm-args=--x86-asm-syntax=intel" "-C" "target-feature=+avx512bw" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-types.i686/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
LLVM ERROR: Bad $ operand number in inline asm string: 'call ${0:c}'
------------------------------------------



---- [assembly] assembly/asm/x86-types.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/x86-types.rs" "-Zthreads=1" "--cfg" "x86_64" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-types.x86_64/x86-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-C" "llvm-args=--x86-asm-syntax=intel" "-C" "target-feature=+avx512bw" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-types.x86_64/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
LLVM ERROR: Bad $ operand number in inline asm string: 'call ${0:c}'
------------------------------------------




failures:
    [assembly] assembly/asm/aarch64-types.rs
    [assembly] assembly/asm/arm-types.rs
    [assembly] assembly/asm/riscv-modifiers.rs
    [assembly] assembly/asm/riscv-types.rs#riscv32
    [assembly] assembly/asm/riscv-types.rs#riscv64
    [assembly] assembly/asm/x86-modifiers.rs#i686
    [assembly] assembly/asm/x86-modifiers.rs#x86_64
    [assembly] assembly/asm/x86-types.rs#i686
    [assembly] assembly/asm/x86-types.rs#x86_64
test result: FAILED. 2 passed; 9 failed; 9 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:02:39
Build completed unsuccessfully in 1:02:39
== clock drift check ==
  local time: Wed Apr 29 21:06:07 UTC 2020
  network time: Wed, 29 Apr 2020 21:06:08 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/69171/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3615) (python)
##[section]Finishing: Finalize Job
