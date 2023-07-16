plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 62'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/74aba268-e711-4d65-a4e1-85e9cbaadef6.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72127/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72127/merge:refs/remotes/pull/72127/merge
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
.....................................................i.............................................. 1800/10161
.................................................................................................... 1900/10161
.......................................................................i..i......................... 2000/10161
.................................................................................................... 2100/10161
.............................................................iiiii.................................. 2200/10161
.................................................................................................... 2400/10161
.................................................................................................... 2500/10161
.................................................................................................... 2600/10161
.................................................................................................... 2700/10161
---
.................................................................................................... 5200/10161
.................................................................................................... 5300/10161
........................i........................................................................... 5400/10161
.................i.................................................................................. 5500/10161
........................ii.ii........i...i.......................................................... 5600/10161
.........................................................................i.......................... 5800/10161
.................................................................................................... 5900/10161
....................ii.....................................i........................................ 6000/10161
.................................................................................................... 6100/10161
.................................................................................................... 6100/10161
.................................................................................................... 6200/10161
.................................................................................ii...i..ii......... 6300/10161
.................................................................................................... 6500/10161
.................................................................................................... 6600/10161
.................................................................................................... 6700/10161
.................................................................................................... 6700/10161
..............i..ii................................................................................. 6800/10161
....FFFF............................................................................................ 6900/10161
....................................................................i............................... 7100/10161
.................................................................................................... 7200/10161
.................................................................................................... 7300/10161
..........i......................................................................................... 7400/10161
---
.................................................................................................... 8100/10161
.................................................................................................... 8200/10161
...................................................................................i................ 8300/10161
.................................................................................................... 8400/10161
.....................................iiiiii.iiiii.i................................................. 8500/10161
.................................................................................................... 8700/10161
.................................................................................................... 8800/10161
.................................................................................................... 8900/10161
.................................................................................................... 9000/10161
---
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-ambiguous/object-lifetime-default-ambiguous.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-ambiguous.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-ambiguous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-ambiguous/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-ambiguous.rs:23:28
   |
LL | fn a<'a,'b>(t: Ref2<'a,'b, dyn Test>) {

error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-ambiguous.rs:27:14
   |
   |
LL | fn b(t: Ref2<dyn Test>) {

error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-ambiguous.rs:43:15
   |
   |
LL | fn f(t: &Ref2<dyn Test>) {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0228`.
---
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic1/object-lifetime-default-dyn-binding-nonstatic1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-dyn-binding-nonstatic1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic1.rs:20:50
   |
LL | fn bar<'a>(x: &'a str) -> &'a dyn Foo<'a, Item = dyn Bar> { &() }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0228`.
---
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic2/object-lifetime-default-dyn-binding-nonstatic2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-dyn-binding-nonstatic2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic2.rs:20:50
   |
LL | fn bar<'a>(x: &'a str) -> &'a dyn Foo<'a, Item = dyn Bar> { &() }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0228`.
---
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3/object-lifetime-default-dyn-binding-nonstatic3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-dyn-binding-nonstatic3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-dyn-binding-nonstatic3.rs:16:36
   |
LL | fn bar(x: &str) -> &dyn Foo<Item = dyn Bar> { &() }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0228`.
---
257 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier/missing-lifetime-specifier.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/missing-lifetime-specifier.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:18:44
   |
LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ expected 2 lifetime parameters
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo<'static, 'static>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:18:44
   |
   |
LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ expected 2 lifetime parameters
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo<'static, 'static>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:44
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&'static Bar>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:45
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ expected 2 lifetime parameters
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar<'static, 'static>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:44
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&'static Bar>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:45
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ expected 2 lifetime parameters
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar<'static, 'static>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:32:48
   |
   |
LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
   |                                                ^ expected 2 lifetime parameters
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:32:48
   |
   |
LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
   |                                                ^ expected 2 lifetime parameters
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:37:44
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&'static Tar<i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:37:49
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   |                                                 ^ expected 2 lifetime parameters
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:37:44
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&'static Tar<i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:37:49
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   |                                                 ^ expected 2 lifetime parameters
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:54:44
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&'static Tar<'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:54:44
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&'static Tar<'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:45
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());

error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:45
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());

error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:37:45
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());

error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:37:45
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());

error[E0107]: wrong number of lifetime arguments: expected 2, found 1
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:47:44
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
error[E0107]: wrong number of lifetime arguments: expected 2, found 1
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:47:44
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
error[E0107]: wrong number of lifetime arguments: expected 2, found 1
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:47:44
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
error[E0107]: wrong number of lifetime arguments: expected 2, found 1
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:47:44
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
error[E0107]: wrong number of lifetime arguments: expected 2, found 1
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:54:45
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
error[E0107]: wrong number of lifetime arguments: expected 2, found 1
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:54:45
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:54:45
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0107]: wrong number of lifetime arguments: expected 2, found 1
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:54:45
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:54:45
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0107]: wrong number of lifetime arguments: expected 2, found 1
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:54:45
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
error: aborting due to 28 previous errors

Some errors have detailed explanations: E0106, E0107, E0228.
For more information about an error, try `rustc --explain E0106`.
---
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore-in-struct/dyn-trait-underscore-in-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args underscore-lifetime/dyn-trait-underscore-in-struct.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/dyn-trait-underscore-in-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore-in-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore-in-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/underscore-lifetime/dyn-trait-underscore-in-struct.rs:9:24
   |
LL |     x: Box<dyn Debug + '_>, //~ ERROR missing lifetime specifier
   |
help: consider introducing a named lifetime parameter
   |
LL | struct Foo<'a> {
LL | struct Foo<'a> {
LL |     x: Box<dyn Debug + 'a>, //~ ERROR missing lifetime specifier

error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
  --> /checkout/src/test/ui/underscore-lifetime/dyn-trait-underscore-in-struct.rs:9:12
   |
   |
LL |     x: Box<dyn Debug + '_>, //~ ERROR missing lifetime specifier

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0106, E0228.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:01:47
Build completed unsuccessfully in 1:01:47
== clock drift check ==
  local time: Tue May 12 07:35:00 UTC 2020
  network time: Tue, 12 May 2020 07:35:00 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72127/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72127/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3591) (python)
##[section]Finishing: Finalize Job
