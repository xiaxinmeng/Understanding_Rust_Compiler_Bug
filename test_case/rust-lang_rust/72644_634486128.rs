plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 40'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/194ffb1a-d72e-4a6b-91fe-4f389b8940fa.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72644/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72644/merge:refs/remotes/pull/72644/merge
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
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
..............................................................................i..................... 1800/10247
.................................................................................................... 1900/10247
.................................................................................................i.. 2000/10247
i................................................................................................... 2100/10247
.......................................................................................iiiii........ 2200/10247
.................................................................................................... 2400/10247
.................................................................................................... 2500/10247
.................................................................................................... 2600/10247
.................................................................................................... 2700/10247
---
...............i...............i.................................................................... 5200/10247
.................................................................................................... 5300/10247
..............................................................i..................................... 5400/10247
.......................................................i............................................ 5500/10247
..................................................................ii.ii........i...i................ 5600/10247
.........i.......................................................................................... 5800/10247
.................i.................................................................................. 5900/10247
......................................................................ii............................ 6000/10247
.........i.......................................................................................... 6100/10247
.........i.......................................................................................... 6100/10247
.................................................................................................... 6200/10247
.................................................................................................... 6300/10247
...............................ii...i..ii...........i............................................... 6400/10247
.................................................................................................... 6600/10247
.................................................................................................... 6700/10247
.................................................................................................... 6700/10247
................................................................i..ii............................... 6800/10247
...........................................................................................F........ 7000/10247
.................................................................................................... 7100/10247
..................i................................................................................. 7200/10247
.................................................................................................... 7300/10247
---
.................................................................................................... 8200/10247
.................................................................................................... 8300/10247
.....................................................i.............................................. 8400/10247
.................................................................................................... 8500/10247
.......iiiiii.iiiiii.i.............................................................................. 8600/10247
.................................................................................................... 8800/10247
.................................................................................................... 8900/10247
.................................................................................................... 9000/10247
.................................................................................................... 9100/10247
---
+    |
+ LL |     let _ = &p.b;
+    |             ^^^^
+    |
+    = note: `#[deny(unaligned_references)]` on by default
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ error: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:29:17
+    |
+    |
+ LL |     let (_,) = (&p.b,);
+    |
+    |
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ error: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:39:11
+    |
+    |
+ LL |     match &p.b  { _ => { } }
+    |
+    |
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ error: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:45:12
+    |
+    |
+ LL |     match (&p.b,)  { (_,) => { } }
+    |
+    |
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
1 warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
2   --> $DIR/issue-53114-safety-checks.rs:23:13
3    |


95    |
96    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
97 
- error: aborting due to 7 previous errors; 4 warnings emitted
+ error: aborting due to 11 previous errors; 4 warnings emitted
100 For more information about this error, try `rustc --explain E0133`.
101 



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
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:23:13
   |
LL |     let _ = &p.b;  //~ WARN    E0133
   |
   = note: `#[deny(unaligned_references)]` on by default
   = note: `#[deny(unaligned_references)]` on by default
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
   |
   |
LL |     let (_,) = (&p.b,);  //~ WARN     E0133
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
   |
   |
LL |     match &p.b  { _ => { } } //~ WARN     E0133
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
   |
   |
LL |     match (&p.b,)  { (_,) => { } } //~ WARN     E0133
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:23:13
   |
   |
LL |     let _ = &p.b;  //~ WARN    E0133
   |
   |
   = note: `#[warn(safe_packed_borrows)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:26:13
   |
   |
LL |     let _ = &u2.a; //~ ERROR  [E0133]
   |             ^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
   |
LL |     let (_,) = (&p.b,);  //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:31:17
   |
   |
LL |     let (_,) = (u1.a,);  //~ ERROR   [E0133]
   |                 ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:32:17
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:32:17
   |
LL |     let (_,) = (&u2.a,); //~ ERROR   [E0133]
   |                 ^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
   |
LL |     match &p.b  { _ => { } } //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:41:11
   |
   |
LL |     match u1.a  { _ => { } } //~ ERROR   [E0133]
   |           ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:42:11
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:42:11
   |
LL |     match &u2.a { _ => { } } //~ ERROR   [E0133]
   |           ^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
   |
LL |     match (&p.b,)  { (_,) => { } } //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:47:12
   |
   |
LL |     match (u1.a,)  { (_,) => { } } //~ ERROR   [E0133]
   |            ^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:48:12
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:48:12
   |
LL |     match (&u2.a,) { (_,) => { } } //~ ERROR   [E0133]
   |            ^^^^^ access to union field
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


error: aborting due to 11 previous errors; 4 warnings emitted
For more information about this error, try `rustc --explain E0133`.

------------------------------------------



---- [ui] ui/issues/issue-27060-rpass.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27060-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060-rpass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060-rpass.rs:16:17
   |
LL |         let _ = &good.data; // ok
   |
   = note: `#[deny(unaligned_references)]` on by default
   = note: `#[deny(unaligned_references)]` on by default
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060-rpass.rs:17:17
   |
   |
LL |         let _ = &good.data2[0]; // ok
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060-rpass.rs:20:13
   |
   |
LL |     let _ = &good.data;
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060-rpass.rs:21:13
   |
   |
LL |     let _ = &good.data2[0];
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: aborting due to 4 previous errors


------------------------------------------
---

+ error: reference to packed field is unaligned
+   --> $DIR/issue-27060.rs:17:17
+    |
+ LL |         let _ = &good.data; // ok
+    |
+    = note: `#[deny(unaligned_references)]` on by default
+    = note: `#[deny(unaligned_references)]` on by default
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ error: reference to packed field is unaligned
+   --> $DIR/issue-27060.rs:18:17
+    |
+    |
+ LL |         let _ = &good.data2[0]; // ok
+    |
+    |
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ error: reference to packed field is unaligned
+   --> $DIR/issue-27060.rs:21:13
+    |
+    |
+ LL |     let _ = &good.data;
+    |
+    |
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ error: reference to packed field is unaligned
+   --> $DIR/issue-27060.rs:23:13
+    |
+    |
+ LL |     let _ = &good.data2[0];
+    |
+    |
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
1 error: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
2   --> $DIR/issue-27060.rs:21:13
3    |


23    = note: for more information, see issue #46043 <***/issues/46043>
24    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
- error: aborting due to 2 previous errors
+ error: aborting due to 6 previous errors
27 
28 
28 


The actual stderr differed from the expected stderr.
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
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060.rs:17:17
   |
LL |         let _ = &good.data; // ok
   |
   = note: `#[deny(unaligned_references)]` on by default
   = note: `#[deny(unaligned_references)]` on by default
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060.rs:18:17
   |
   |
LL |         let _ = &good.data2[0]; // ok
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060.rs:21:13
   |
   |
LL |     let _ = &good.data; //~ ERROR borrow of packed field is unsafe
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060.rs:23:13
   |
   |
LL |     let _ = &good.data2[0]; //~ ERROR borrow of packed field is unsafe
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
  --> /checkout/src/test/ui/issues/issue-27060.rs:21:13
   |
   |
LL |     let _ = &good.data; //~ ERROR borrow of packed field is unsafe
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/issues/issue-27060.rs:8:8
   |
   |
LL | #[deny(safe_packed_borrows)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
  --> /checkout/src/test/ui/issues/issue-27060.rs:23:13
   |
   |
LL |     let _ = &good.data2[0]; //~ ERROR borrow of packed field is unsafe
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #46043 <***/issues/46043>
   = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error: aborting due to 6 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/packed/packed-struct-borrow-element.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/packed-struct-borrow-element.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-borrow-element/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-borrow-element/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/packed-struct-borrow-element.rs:25:24
   |
LL |     let brw = unsafe { &foo.baz };
   |
   = note: `#[deny(unaligned_references)]` on by default
   = note: `#[deny(unaligned_references)]` on by default
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/packed-struct-borrow-element.rs:29:24
   |
   |
LL |     let brw = unsafe { &foo.baz };
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/packed-struct-borrow-element.rs:33:24
   |
   |
LL |     let brw = unsafe { &foo.baz };
   |
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: aborting due to 3 previous errors


------------------------------------------
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:05:45
Build completed unsuccessfully in 1:05:45
== clock drift check ==
  local time: Wed May 27 07:38:06 UTC 2020
  network time: Wed, 27 May 2020 07:38:06 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72644/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72644/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3967) (python)
##[section]Finishing: Finalize Job
