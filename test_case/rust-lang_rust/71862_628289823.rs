plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 10'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200430.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200430.2/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/99ca2143-edde-42c5-890b-0e8fbcaba055.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71862/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71862/merge:refs/remotes/pull/71862/merge
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
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
......................................................i............................................. 1800/10166
............................................................F....................................... 1900/10166
........................................................................i..i........................ 2000/10166
.................................................................................................... 2100/10166
..............................................................iiiii................................. 2200/10166
.................................................................................................... 2400/10166
...F................................................................................................ 2500/10166
.................................................................................................... 2600/10166
.................................................................................................... 2700/10166
---
.................................................................................................... 3200/10166
.i.................................................................................................. 3300/10166
.................................................................................................... 3400/10166
.................................................................................................... 3500/10166
............ii.................................................F.F.................................. 3600/10166
...................................F................................................................ 3800/10166
........i.................F......................................................................... 3900/10166
.................................................................................................... 4000/10166
.................................................................................................... 4100/10166
---
.................................................................................................... 5200/10166
.................................................................................................... 5300/10166
........F.................i......................................................................... 5400/10166
...................i................................................................................ 5500/10166
..........................ii.ii........i...i........................................................ 5600/10166
...........................................................................i........................ 5800/10166
.................................................................................................... 5900/10166
......................ii.....................................i...................................... 6000/10166
.................................................................................................... 6100/10166
.................................................................................................... 6100/10166
.................................................................................................... 6200/10166
...................................................................................ii...i..ii....... 6300/10166
.................................................................................................... 6500/10166
.................................................................................................... 6600/10166
.................................................................................................... 6700/10166
.................................................................................................... 6700/10166
................i..ii............................................................................... 6800/10166
.................................................................................................... 7000/10166
......................................................................i............................. 7100/10166
.................................................................................................... 7200/10166
.................................................................................................... 7300/10166
---
.................................................................................................... 8100/10166
.................................................................................................... 8200/10166
.......................................................................................i............ 8300/10166
....................................................................F............................... 8400/10166
........................................Fiiiiii.iiiii.iF............................................ 8500/10166
.................................................................................................... 8700/10166
.................................................................................................... 8800/10166
......F...F......................................................................................... 8900/10166
.................................................................................................... 9000/10166
---
....................................................................i............................... 9600/10166
.................................................................................................... 9700/10166
.................................................................................................... 9800/10166
...................................................F................................................ 9900/10166
...............F.FFFFFFF...FF.FFFF.................................................................. 10000/10166
..................................................................
failures:
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


---- [ui] ui/async-await/async-unsafe-fn-call-in-safe.rs stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
2   --> $DIR/async-unsafe-fn-call-in-safe.rs:12:5
3    |
4 LL |     S::f();
6    |
7    = note: consult the function's documentation for information on how to avoid undefined behavior
8 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
18   --> $DIR/async-unsafe-fn-call-in-safe.rs:17:5
19    |
20 LL |     S::f();
22    |
23    = note: consult the function's documentation for information on how to avoid undefined behavior
24 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
26   --> $DIR/async-unsafe-fn-call-in-safe.rs:18:5
27    |
28 LL |     f();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe/async-unsafe-fn-call-in-safe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/async-unsafe-fn-call-in-safe.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:12:5
   |
LL |     S::f(); //~ ERROR call to unsafe function is unsafe
   |     ^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:13:5
---

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:17:5
   |
LL |     S::f(); //~ ERROR call to unsafe function is unsafe
   |     ^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:18:5
---
2   --> $DIR/issue-53114-safety-checks.rs:23:13
3    |
4 LL |     let _ = &p.b;

9    = note: for more information, see issue #46043 <***/issues/46043>
10    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
13   --> $DIR/issue-53114-safety-checks.rs:26:13
14    |
---
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
+ warning: borrow of packed field is unsafe and requires unsafe block (error E0133)
21   --> $DIR/issue-53114-safety-checks.rs:29:17
22    |
23 LL |     let (_,) = (&p.b,);

27    = note: for more information, see issue #46043 <***/issues/46043>
28    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
31   --> $DIR/issue-53114-safety-checks.rs:31:17
32    |
32    |
33 LL |     let (_,) = (u1.a,);
35    |
36    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
37 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
39   --> $DIR/issue-53114-safety-checks.rs:32:17
40    |
41 LL |     let (_,) = (&u2.a,);
43    |
44    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
45 
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
+ warning: borrow of packed field is unsafe and requires unsafe block (error E0133)
47   --> $DIR/issue-53114-safety-checks.rs:39:11
48    |
49 LL |     match &p.b  { _ => { } }

53    = note: for more information, see issue #46043 <***/issues/46043>
54    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
57   --> $DIR/issue-53114-safety-checks.rs:41:11
58    |
58    |
59 LL |     match u1.a  { _ => { } }
61    |
62    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
63 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
65   --> $DIR/issue-53114-safety-checks.rs:42:11
66    |
67 LL |     match &u2.a { _ => { } }
69    |
70    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
71 
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
+ warning: borrow of packed field is unsafe and requires unsafe block (error E0133)
73   --> $DIR/issue-53114-safety-checks.rs:45:12
74    |
75 LL |     match (&p.b,)  { (_,) => { } }

79    = note: for more information, see issue #46043 <***/issues/46043>
80    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
83   --> $DIR/issue-53114-safety-checks.rs:47:12
84    |
84    |
85 LL |     match (u1.a,)  { (_,) => { } }
87    |
88    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
89 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
91   --> $DIR/issue-53114-safety-checks.rs:48:12
92    |
93 LL |     match (&u2.a,) { (_,) => { } }

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks/issue-53114-safety-checks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binding/issue-53114-safety-checks.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/issue-53114-safety-checks.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: borrow of packed field is unsafe and requires unsafe block (error E0133)
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:23:13
   |
LL |     let _ = &p.b;  //~ WARN    E0133
   |
   |
   = note: `#[warn(safe_packed_borrows)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:26:13
   |
   |
LL |     let _ = &u2.a; //~ ERROR  [E0133]
   |             ^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

warning: borrow of packed field is unsafe and requires unsafe block (error E0133)
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
   |
LL |     let (_,) = (&p.b,);  //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:31:17
   |
   |
LL |     let (_,) = (u1.a,);  //~ ERROR   [E0133]
   |                 ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:32:17
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:32:17
   |
LL |     let (_,) = (&u2.a,); //~ ERROR   [E0133]
   |                 ^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

warning: borrow of packed field is unsafe and requires unsafe block (error E0133)
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
   |
LL |     match &p.b  { _ => { } } //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:41:11
   |
   |
LL |     match u1.a  { _ => { } } //~ ERROR   [E0133]
   |           ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:42:11
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:42:11
   |
LL |     match &u2.a { _ => { } } //~ ERROR   [E0133]
   |           ^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

warning: borrow of packed field is unsafe and requires unsafe block (error E0133)
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
   |
LL |     match (&p.b,)  { (_,) => { } } //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:47:12
   |
   |
LL |     match (u1.a,)  { (_,) => { } } //~ ERROR   [E0133]
   |            ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:48:12
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:48:12
   |
LL |     match (&u2.a,) { (_,) => { } } //~ ERROR   [E0133]
   |            ^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


error: aborting due to 7 previous errors; 4 warnings emitted
For more information about this error, try `rustc --explain E0133`.

------------------------------------------

---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
10   --> $DIR/const-extern-fn-requires-unsafe.rs:6:17
11    |
12 LL |     let a: [u8; foo()];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe/const-extern-fn-requires-unsafe.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe/const-extern-fn-requires-unsafe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-extern-fn/const-extern-fn-requires-unsafe.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs:6:17
   |
LL |     let a: [u8; foo()];
   |                 ^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to 2 previous errors

---

---- [ui] ui/consts/min_const_fn/min_const_fn_unsafe_bad.rs stdout ----
diff of stderr:

34    = note: see issue #57563 <***/issues/57563> for more information
35    = help: add `#![feature(const_fn)]` to the crate attributes to enable
- error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
+ error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
38   --> $DIR/min_const_fn_unsafe_bad.rs:1:77
39    |
39    |
40 LL | const fn bad_const_fn_deref_raw(x: *mut usize) -> &'static usize { unsafe { &*x } }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad/min_const_fn_unsafe_bad.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad/min_const_fn_unsafe_bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_unsafe_bad.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: dereferencing raw pointers in constant functions is unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad.rs:1:77
   |
LL | const fn bad_const_fn_deref_raw(x: *mut usize) -> &'static usize { unsafe { &*x } } //~ is unsafe
   |
   |
   = note: see issue #51911 <***/issues/51911> for more information

error[E0658]: dereferencing raw pointers in constant functions is unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad.rs:4:70
   |
   |
LL | const unsafe fn bad_const_unsafe_deref_raw(x: *mut usize) -> usize { *x }
   |
   |
   = note: see issue #51911 <***/issues/51911> for more information

error[E0658]: dereferencing raw pointers in constant functions is unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad.rs:7:83
   |
   |
LL | const unsafe fn bad_const_unsafe_deref_raw_ref(x: *mut usize) -> &'static usize { &*x }
   |
   |
   = note: see issue #51911 <***/issues/51911> for more information

error[E0723]: accessing union fields is unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad.rs:14:5
   |
   |
LL |     Foo { x: () }.y
   |     ^^^^^^^^^^^^^^^
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe_bad.rs:1:77
   |
   |
LL | const fn bad_const_fn_deref_raw(x: *mut usize) -> &'static usize { unsafe { &*x } } //~ is unsafe
   |                                                                             ^^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0133, E0658, E0723.
For more information about an error, try `rustc --explain E0133`.
---
4 LL |     f();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0133/E0133.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0133.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0133.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0133" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0133/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---


---- [ui] ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs stdout ----

error: /checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs:1: unexpected error: '1:1: 1:33: the `unsafe_op_in_unsafe_fn` lint is unstable [E0658]'

error: /checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs:1: unexpected error: '1:1: 1:33: the `unsafe_op_in_unsafe_fn` lint is unstable [E0658]'
error: 2 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "1:1: 1:33: the `unsafe_op_in_unsafe_fn` lint is unstable [E0658]",
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "1:1: 1:33: the `unsafe_op_in_unsafe_fn` lint is unstable [E0658]",
]

thread '[ui] ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
4 LL |     test::free();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign-unsafe-fn-called/foreign-unsafe-fn-called.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args foreign-unsafe-fn-called.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/foreign-unsafe-fn-called.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign-unsafe-fn-called" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign-unsafe-fn-called/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/issue-28575/issue-28575.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intrinsics/issue-28575.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/intrinsics/issue-28575.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/issue-28575" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/issue-28575/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/intrinsics/issue-28575.rs:8:5
   |
LL |     FOO() //~ ERROR: use of extern static is unsafe
   |     ^^^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
2   --> $DIR/unchecked_math_unsafe.rs:5:15
3    |
4 LL |     let add = std::intrinsics::unchecked_add(x, y);
6    |
7    = note: consult the function's documentation for information on how to avoid undefined behavior
8 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
10   --> $DIR/unchecked_math_unsafe.rs:6:15
11    |
12 LL |     let sub = std::intrinsics::unchecked_sub(x, y);
14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
16 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
18   --> $DIR/unchecked_math_unsafe.rs:7:15
19    |
20 LL |     let mul = std::intrinsics::unchecked_mul(x, y);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/unchecked_math_unsafe/unchecked_math_unsafe.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/unchecked_math_unsafe/unchecked_math_unsafe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intrinsics/unchecked_math_unsafe.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/intrinsics/unchecked_math_unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/unchecked_math_unsafe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/unchecked_math_unsafe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/intrinsics/unchecked_math_unsafe.rs:5:15
   |
LL |     let add = std::intrinsics::unchecked_add(x, y); //~ ERROR call to unsafe function
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/intrinsics/unchecked_math_unsafe.rs:6:15
  --> /checkout/src/test/ui/intrinsics/unchecked_math_unsafe.rs:6:15
   |
LL |     let sub = std::intrinsics::unchecked_sub(x, y); //~ ERROR call to unsafe function
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/intrinsics/unchecked_math_unsafe.rs:7:15
  --> /checkout/src/test/ui/intrinsics/unchecked_math_unsafe.rs:7:15
   |
LL |     let mul = std::intrinsics::unchecked_mul(x, y); //~ ERROR call to unsafe function
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to 3 previous errors

---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14227/issue-14227.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-14227.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14227.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14227" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14227/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-14227.rs:4:21
   |
LL | static CRASH: u32 = symbol;
   |                     ^^^^^^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
- error[E0133]: use of extern static is unsafe and requires unsafe function or block
+ error[E0133]: use of extern static is unsafe and requires unsafe block
17   --> $DIR/issue-16538.rs:11:34
18    |
19 LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538/issue-16538.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538/issue-16538.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-16538.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16538.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/issues/issue-16538.rs:11:27
   |
LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);

error[E0277]: `*const usize` cannot be shared between threads safely
  --> /checkout/src/test/ui/issues/issue-16538.rs:11:1
   |
   |
LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `*const usize` cannot be shared between threads safely
   = help: the trait `std::marker::Sync` is not implemented for `*const usize`
   = note: shared static variables must have a type that implements `Sync`

error[E0133]: use of extern static is unsafe and requires unsafe block
error[E0133]: use of extern static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-16538.rs:11:34
   |
LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);
   |
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0015, E0133, E0277.
For more information about an error, try `rustc --explain E0015`.
---
- error: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
+ error: borrow of packed field is unsafe and requires unsafe block (error E0133)
2   --> $DIR/issue-27060.rs:26:13
3    |
4 LL |     let _ = &good.data;

13    = note: for more information, see issue #46043 <***/issues/46043>
14    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
- error: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
+ error: borrow of packed field is unsafe and requires unsafe block (error E0133)
17   --> $DIR/issue-27060.rs:28:13
18    |
18    |
19 LL |     let _ = &good.data2[0];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060/issue-27060.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060/issue-27060.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-27060.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: borrow of packed field is unsafe and requires unsafe block (error E0133)
  --> /checkout/src/test/ui/issues/issue-27060.rs:26:13
   |
LL |     let _ = &good.data; //~ ERROR borrow of packed field is unsafe
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/issues/issue-27060.rs:13:8
   |
   |
LL | #[deny(safe_packed_borrows)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error: borrow of packed field is unsafe and requires unsafe block (error E0133)
  --> /checkout/src/test/ui/issues/issue-27060.rs:28:13
   |
   |
LL |     let _ = &good.data2[0]; //~ ERROR borrow of packed field is unsafe
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error: aborting due to 2 previous errors


------------------------------------------
---
- error[E0133]: use of extern static is unsafe and requires unsafe function or block
+ error[E0133]: use of extern static is unsafe and requires unsafe block
2   --> $DIR/issue-28324.rs:5:24
3    |
4 LL | pub static BAZ: u32 = *&error_message_count;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28324/issue-28324.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28324/issue-28324.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-28324.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28324.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28324" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28324/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-28324.rs:5:24
   |
LL | pub static BAZ: u32 = *&error_message_count;
   |
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
2   --> $DIR/issue-28776.rs:4:5
3    |
4 LL |     (&ptr::write)(1 as *mut _, 42);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28776/issue-28776.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28776/issue-28776.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-28776.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28776.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28776" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28776/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-28776.rs:4:5
   |
LL |     (&ptr::write)(1 as *mut _, 42);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error

---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
2   --> $DIR/issue-3080.rs:7:5
3    |
4 LL |     X(()).with();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3080/issue-3080.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3080/issue-3080.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-3080.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3080.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3080" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3080/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-3080.rs:7:5
   |
LL |     X(()).with(); //~ ERROR requires unsafe function or block
   |     ^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error

---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
10   --> $DIR/issue-43733.rs:22:5
11    |
12 LL |     std::thread::LocalKey::new(__getit);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43733/issue-43733.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43733/issue-43733.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-43733.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43733.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43733" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43733/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-43733.rs:18:5
   |
LL |     __KEY.get(Default::default) //~ ERROR call to unsafe function is unsafe
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-43733.rs:22:5
  --> /checkout/src/test/ui/issues/issue-43733.rs:22:5
   |
LL |     std::thread::LocalKey::new(__getit);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to 2 previous errors

---
- error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
+ error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
2   --> $DIR/issue-45087-unreachable-unsafe.rs:3:5
3    |
4 LL |     *(1 as *mut u32) = 42;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45087-unreachable-unsafe/issue-45087-unreachable-unsafe.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45087-unreachable-unsafe/issue-45087-unreachable-unsafe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-45087-unreachable-unsafe.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45087-unreachable-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45087-unreachable-unsafe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45087-unreachable-unsafe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-45087-unreachable-unsafe.rs:3:5
   |
LL |     *(1 as *mut u32) = 42;
   |     ^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
- error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
+ error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
2   --> $DIR/issue-45729-unsafe-in-generator.rs:5:9
3    |
4 LL |         *(1 as *mut u32) = 42;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45729-unsafe-in-generator/issue-45729-unsafe-in-generator.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45729-unsafe-in-generator/issue-45729-unsafe-in-generator.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-45729-unsafe-in-generator.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45729-unsafe-in-generator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45729-unsafe-in-generator" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45729-unsafe-in-generator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-45729-unsafe-in-generator.rs:5:9
   |
LL |         *(1 as *mut u32) = 42;
   |         ^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
2   --> $DIR/issue-47412.rs:11:11
3    |
4 LL |     match u.void {}
6    |
7    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
8 
- error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47412/issue-47412.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-47412.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47412.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47412" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47412/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-47412.rs:11:11
   |
LL |     match u.void {}
   |           ^^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-47412.rs:17:11
  --> /checkout/src/test/ui/issues/issue-47412.rs:17:11
   |
LL |     match *ptr {}
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.

---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
2   --> $DIR/issue-5844.rs:6:5
3    |
4 LL |     issue_5844_aux::rand();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5844/issue-5844.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5844/issue-5844.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-5844.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5844.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5844" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5844/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/issues/issue-5844.rs:6:5
   |
LL |     issue_5844_aux::rand(); //~ ERROR: requires unsafe
   |     ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error


For more information about this error, try `rustc --explain E0133`.

------------------------------------------


---- [ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs stdout ----

- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
2   --> $DIR/safe-calls.rs:21:5
---
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
10   --> $DIR/safe-calls.rs:22:5
11    |
12 LL |     avx_bmi2();
14    |
15    = note: can only be called if the required target features are available
16 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
18   --> $DIR/safe-calls.rs:23:5
19    |
20 LL |     Quux.avx_bmi2();
22    |
23    = note: can only be called if the required target features are available
24 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
26   --> $DIR/safe-calls.rs:28:5
27    |
28 LL |     avx_bmi2();
30    |
31    = note: can only be called if the required target features are available
32 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
34   --> $DIR/safe-calls.rs:29:5
35    |
36 LL |     Quux.avx_bmi2();
38    |
39    = note: can only be called if the required target features are available
40 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
---
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
50   --> $DIR/safe-calls.rs:35:5
51    |
52 LL |     avx_bmi2();
54    |
55    = note: can only be called if the required target features are available
56 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
58   --> $DIR/safe-calls.rs:36:5
59    |
60 LL |     Quux.avx_bmi2();
62    |
63    = note: can only be called if the required target features are available
64 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
---
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
74   --> $DIR/safe-calls.rs:45:18
75    |
76 LL | const name: () = sse2();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls/safe-calls.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/safe-calls.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:21:5
   |
LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:22:5
   |
LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:23:5
   |
LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:28:5
   |
LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:29:5
   |
LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:34:5
   |
LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:35:5
   |
LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:36:5
   |
LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:42:5
   |
LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe block
  --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:45:18
   |
LL | const name: () = sse2(); //~ ERROR call to function with `#[target_feature]` is unsafe
   |                  ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error: aborting due to 10 previous errors

---
- error[E0133]: use of mutable static is unsafe and requires unsafe function or block
+ error[E0133]: use of mutable static is unsafe and requires unsafe block
10   --> $DIR/safe-extern-statics-mut.rs:12:14
11    |
12 LL |     let rb = &B;
14    |
15    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
16 
- error[E0133]: use of mutable static is unsafe and requires unsafe function or block
- error[E0133]: use of mutable static is unsafe and requires unsafe function or block
+ error[E0133]: use of mutable static is unsafe and requires unsafe block
18   --> $DIR/safe-extern-statics-mut.rs:13:14
19    |
20 LL |     let xb = XB;
22    |
23    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
24 
- error[E0133]: use of mutable static is unsafe and requires unsafe function or block
- error[E0133]: use of mutable static is unsafe and requires unsafe function or block
+ error[E0133]: use of mutable static is unsafe and requires unsafe block
26   --> $DIR/safe-extern-statics-mut.rs:14:15
27    |
28 LL |     let xrb = &XB;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics-mut/safe-extern-statics-mut.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics-mut/safe-extern-statics-mut.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args safe-extern-statics-mut.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/safe-extern-statics-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics-mut/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0133]: use of mutable static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/safe-extern-statics-mut.rs:12:14
   |
LL |     let rb = &B; //~ ERROR use of mutable static is unsafe
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe block
error[E0133]: use of mutable static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/safe-extern-statics-mut.rs:13:14
   |
LL |     let xb = XB; //~ ERROR use of mutable static is unsafe
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe block
error[E0133]: use of mutable static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/safe-extern-statics-mut.rs:14:15
   |
LL |     let xrb = &XB; //~ ERROR use of mutable static is unsafe
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: aborting due to 4 previous errors
---
3    |
4 LL |     let a = A;

6    |
7    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
- error[E0133]: use of extern static is unsafe and requires unsafe function or block
+ error[E0133]: use of extern static is unsafe and requires unsafe block
10   --> $DIR/safe-extern-statics.rs:12:14
11    |
11    |
12 LL |     let ra = &A;
14    |
14    |
15    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
- error[E0133]: use of extern static is unsafe and requires unsafe function or block
+ error[E0133]: use of extern static is unsafe and requires unsafe block
18   --> $DIR/safe-extern-statics.rs:13:14
19    |
19    |
20 LL |     let xa = XA;
22    |
22    |
23    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
- error[E0133]: use of extern static is unsafe and requires unsafe function or block
+ error[E0133]: use of extern static is unsafe and requires unsafe block
26   --> $DIR/safe-extern-statics.rs:14:15
27    |
27    |
28 LL |     let xra = &XA;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics/safe-extern-statics.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics/safe-extern-statics.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args safe-extern-statics.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/safe-extern-statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/safe-extern-statics.rs:11:13
   |
LL |     let a = A; //~ ERROR use of extern static is unsafe
   |             ^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error[E0133]: use of extern static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/safe-extern-statics.rs:12:14
   |
   |
LL |     let ra = &A; //~ ERROR use of extern static is unsafe
   |
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error[E0133]: use of extern static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/safe-extern-statics.rs:13:14
   |
   |
LL |     let xa = XA; //~ ERROR use of extern static is unsafe
   |
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error[E0133]: use of extern static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/safe-extern-statics.rs:14:15
   |
   |
LL |     let xra = &XA; //~ ERROR use of extern static is unsafe
   |
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.

---
20 LL |     let _b = a;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-foreign-requires-unsafe/static-mut-foreign-requires-unsafe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-mut-foreign-requires-unsafe.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-mut-foreign-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-foreign-requires-unsafe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-foreign-requires-unsafe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0133]: use of mutable static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/static/static-mut-foreign-requires-unsafe.rs:8:14
   |
LL |     let _b = a; //~ ERROR: requires unsafe
   |              ^ use of mutable static
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: aborting due to 3 previous errors

---
20 LL |     let _b = a;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-requires-unsafe/static-mut-requires-unsafe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-mut-requires-unsafe.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-mut-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-requires-unsafe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-requires-unsafe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0133]: use of mutable static is unsafe and requires unsafe block
  --> /checkout/src/test/ui/static/static-mut-requires-unsafe.rs:6:14
   |
LL |     let _b = a;     //~ ERROR: requires unsafe
   |              ^ use of mutable static
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: aborting due to 3 previous errors

---
4 LL |         *self += 1;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-safety-fn-body/trait-safety-fn-body.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/trait-safety-fn-body.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-safety-fn-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-safety-fn-body" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-safety-fn-body/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> /checkout/src/test/ui/traits/trait-safety-fn-body.rs:11:9
   |
LL |         *self += 1;
   |         ^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/union/union-unsafe.rs stdout ----
diff of stderr:

- error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe function or block
+ error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe block
3    |
3    |
4 LL |     u3.a = ManuallyDrop::new(T::default());
6    |
7    = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized
8 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
10   --> $DIR/union-unsafe.rs:23:6
11    |
12 LL |     *u3.a = T::default();
14    |
15    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
16 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
18   --> $DIR/union-unsafe.rs:29:6
19    |
20 LL |     *u3.a = T::default();
22    |
23    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
24 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
---
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
34   --> $DIR/union-unsafe.rs:40:14
35    |
36 LL |     let U1 { a } = u1;
38    |
39    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
40 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
42   --> $DIR/union-unsafe.rs:41:20
43    |
44 LL |     if let U1 { a: 12 } = u1 {}
46    |
47    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
48 
48 
- error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe function or block
+ error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe block
51    |
51    |
52 LL |     u2.a = ManuallyDrop::new(String::from("new"));
54    |
55    = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized
56 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
58   --> $DIR/union-unsafe.rs:46:6
59    |
60 LL |     *u2.a = String::from("new");
62    |
63    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
64 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
66   --> $DIR/union-unsafe.rs:50:6
67    |
68 LL |     *u3.a = 1;
70    |
71    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
72 
72 
- error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe function or block
+ error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe block
75    |
75    |
76 LL |     u3.a = ManuallyDrop::new(String::from("new"));
78    |
79    = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized
80 
- error[E0133]: access to union field is unsafe and requires unsafe function or block
- error[E0133]: access to union field is unsafe and requires unsafe function or block
+ error[E0133]: access to union field is unsafe and requires unsafe block
82   --> $DIR/union-unsafe.rs:54:6
83    |
84 LL |     *u3.a = String::from("new");

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe/union-unsafe.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe/union-unsafe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args union/union-unsafe.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe block
   |
   |
LL |     u3.a = ManuallyDrop::new(T::default()); //~ ERROR assignment to non-`Copy` union field is unsafe
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to non-`Copy` union field
   = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/union/union-unsafe.rs:23:6
  --> /checkout/src/test/ui/union/union-unsafe.rs:23:6
   |
LL |     *u3.a = T::default(); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/union/union-unsafe.rs:29:6
  --> /checkout/src/test/ui/union/union-unsafe.rs:29:6
   |
LL |     *u3.a = T::default(); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/union/union-unsafe.rs:37:13
  --> /checkout/src/test/ui/union/union-unsafe.rs:37:13
   |
LL |     let a = u1.a; //~ ERROR access to union field is unsafe
   |             ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/union/union-unsafe.rs:40:14
  --> /checkout/src/test/ui/union/union-unsafe.rs:40:14
   |
LL |     let U1 { a } = u1; //~ ERROR access to union field is unsafe
   |              ^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/union/union-unsafe.rs:41:20
  --> /checkout/src/test/ui/union/union-unsafe.rs:41:20
   |
LL |     if let U1 { a: 12 } = u1 {} //~ ERROR access to union field is unsafe
   |                    ^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe block
   |
   |
LL |     u2.a = ManuallyDrop::new(String::from("new")); //~ ERROR assignment to non-`Copy` union
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to non-`Copy` union field
   = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/union/union-unsafe.rs:46:6
  --> /checkout/src/test/ui/union/union-unsafe.rs:46:6
   |
LL |     *u2.a = String::from("new"); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/union/union-unsafe.rs:50:6
  --> /checkout/src/test/ui/union/union-unsafe.rs:50:6
   |
LL |     *u3.a = 1; //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


error[E0133]: assignment to non-`Copy` union field is unsafe and requires unsafe block
   |
   |
LL |     u3.a = ManuallyDrop::new(String::from("new")); //~ ERROR assignment to non-`Copy` union
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to non-`Copy` union field
   = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized

error[E0133]: access to union field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/union/union-unsafe.rs:54:6
  --> /checkout/src/test/ui/union/union-unsafe.rs:54:6
   |
LL |     *u3.a = String::from("new"); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error: aborting due to 11 previous errors

---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints/ranged_ints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/ranged_ints.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2/ranged_ints2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/ranged_ints2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: mutation of layout constrained field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/ranged_ints2.rs:8:13
   |
LL |     let y = &mut x.0; //~ ERROR mutation of layout constrained field is unsafe
   |             ^^^^^^^^ mutation of layout constrained field
   = note: mutating layout constrained fields cannot statically be checked for valid values

error: aborting due to previous error

---

---- [ui] ui/unsafe/ranged_ints2_const.rs stdout ----
diff of stderr:

16    = note: see issue #57563 <***/issues/57563> for more information
17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
- error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block
+ error[E0133]: mutation of layout constrained field is unsafe and requires unsafe block
20   --> $DIR/ranged_ints2_const.rs:11:13
21    |
21    |
22 LL |     let y = &mut x.0;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/ranged_ints2_const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/ranged_ints2_const.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints2_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0723]: mutable references in const fn are unstable
  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:11:9
   |
LL |     let y = &mut x.0; //~ ERROR references in const fn are unstable
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0723]: mutable references in const fn are unstable
  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:18:9
   |
   |
LL |     let y = unsafe { &mut x.0 }; //~ ERROR mutable references in const fn are unstable
   |
   |
   = note: see issue #57563 <***/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
error[E0133]: mutation of layout constrained field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:11:13
   |
   |
LL |     let y = &mut x.0; //~ ERROR references in const fn are unstable
   |             ^^^^^^^^ mutation of layout constrained field
   = note: mutating layout constrained fields cannot statically be checked for valid values

error: aborting due to 3 previous errors

---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3/ranged_ints3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/ranged_ints3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/ranged_ints3.rs:10:13
   |
LL |     let y = &x.0; //~ ERROR borrow of layout constrained field with interior mutability
   |             ^^^^ borrow of layout constrained field with interior mutability
   = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values

error: aborting due to previous error

---

---- [ui] ui/unsafe/ranged_ints3_const.rs stdout ----
diff of stderr:

10 LL |     let y = unsafe { &x.0 };
12 
- error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block
+ error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe block
14   --> $DIR/ranged_ints3_const.rs:12:13
14   --> $DIR/ranged_ints3_const.rs:12:13
15    |
16 LL |     let y = &x.0;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const/ranged_ints3_const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/ranged_ints3_const.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints3_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
  --> /checkout/src/test/ui/unsafe/ranged_ints3_const.rs:12:13
   |
LL |     let y = &x.0; //~ ERROR cannot borrow a constant which may contain interior mutability

error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
  --> /checkout/src/test/ui/unsafe/ranged_ints3_const.rs:19:22
   |
   |
LL |     let y = unsafe { &x.0 }; //~ ERROR cannot borrow a constant which may contain interior mut

error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/ranged_ints3_const.rs:12:13
   |
   |
LL |     let y = &x.0; //~ ERROR cannot borrow a constant which may contain interior mutability
   |             ^^^^ borrow of layout constrained field with interior mutability
   = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values

error: aborting due to 3 previous errors

---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints4/ranged_ints4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/ranged_ints4.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: mutation of layout constrained field is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/ranged_ints4.rs:8:5
   |
LL |     x.0 = 0; //~ ERROR mutation of layout constrained field is unsafe
   |     ^^^^^^^ mutation of layout constrained field
   = note: mutating layout constrained fields cannot statically be checked for valid values

error: aborting due to previous error

---
4 LL |     x.0 = 0;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints4_const/ranged_ints4_const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/ranged_ints4_const.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints4_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints4_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints4_const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
- error[E0133]: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe function or block
+ error[E0133]: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe block
2   --> $DIR/ranged_ints_const.rs:8:34
3    |
4 LL | const fn foo() -> NonZero<u32> { NonZero(0) }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints_const/ranged_ints_const.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints_const/ranged_ints_const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/ranged_ints_const.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints_const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: initializing type with `rustc_layout_scalar_valid_range` attr is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/ranged_ints_const.rs:8:34
   |
LL | const fn foo() -> NonZero<u32> { NonZero(0) }
   |                                  ^^^^^^^^^^ initializing type with `rustc_layout_scalar_valid_range` attr
   = note: initializing a layout restricted type's field with a value outside the valid range is undefined behavior

error: aborting due to previous error

---
4 LL |     *p = 0;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr/unsafe-fn-assign-deref-ptr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/unsafe-fn-assign-deref-ptr.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-assign-deref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/unsafe-fn-assign-deref-ptr.rs:6:5
   |
LL |     *p = 0; //~ ERROR dereference of raw pointer is unsafe
   |     ^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function is unsafe and requires unsafe block
2   --> $DIR/unsafe-const-fn.rs:7:18
3    |
4 LL | const VAL: u32 = dummy(0xFFFF);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-const-fn/unsafe-const-fn.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-const-fn/unsafe-const-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/unsafe-const-fn.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-const-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-const-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/unsafe-const-fn.rs:7:18
   |
LL | const VAL: u32 = dummy(0xFFFF);
   |                  ^^^^^^^^^^^^^ call to unsafe function
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error

---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-called-from-safe/unsafe-fn-called-from-safe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/unsafe-fn-called-from-safe.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-called-from-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-called-from-safe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-called-from-safe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
4 LL |     return *p;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-deref-ptr/unsafe-fn-deref-ptr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/unsafe-fn-deref-ptr.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-deref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-deref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-deref-ptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/unsafe-fn-deref-ptr.rs:2:12
   |
LL |     return *p; //~ ERROR dereference of raw pointer is unsafe
   |            ^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
4 LL |     x();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-used-as-value/unsafe-fn-used-as-value.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/unsafe-fn-used-as-value.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-used-as-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-used-as-value" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-used-as-value/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
- error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
+ error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
2   --> $DIR/unsafe-move-val-init.rs:8:5
3    |
4 LL |     intrinsics::move_val_init(1 as *mut u32, 1);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-move-val-init/unsafe-move-val-init.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-move-val-init/unsafe-move-val-init.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/unsafe-move-val-init.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-move-val-init.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-move-val-init" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-move-val-init/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/unsafe-move-val-init.rs:8:5
   |
LL |     intrinsics::move_val_init(1 as *mut u32, 1);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
    [ui] ui/issues/issue-45087-unreachable-unsafe.rs
    [ui] ui/issues/issue-45729-unsafe-in-generator.rs
    [ui] ui/issues/issue-47412.rs
    [ui] ui/issues/issue-5844.rs
    [ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs
    [ui] ui/safe-extern-statics.rs
    [ui] ui/static/static-mut-foreign-requires-unsafe.rs
    [ui] ui/static/static-mut-requires-unsafe.rs
    [ui] ui/traits/trait-safety-fn-body.rs
---
test result: FAILED. 10064 passed; 41 failed; 61 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:59
Build completed unsuccessfully in 1:05:59
== clock drift check ==
  local time: Wed May 13 23:01:54 UTC 2020
  network time: Wed, 13 May 2020 23:01:54 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71862/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71862/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3763) (python)
##[section]Finishing: Finalize Job
