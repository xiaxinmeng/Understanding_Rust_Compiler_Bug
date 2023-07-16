plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 9'
Agent machine name: 'fv-az578'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/78498420-8a13-4ba4-ba95-66e9ee27cc8d.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72804/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72804/merge:refs/remotes/pull/72804/merge
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
.................................................................................................... 1700/10280
...................................................................................i................ 1800/10280
.................................................................................................... 1900/10280
.................................................................................................... 2000/10280
.....i..i........................................................................................... 2100/10280
...............................................................................................iiiii 2200/10280
.................................................................................................... 2400/10280
.................................................................................................... 2500/10280
.................................................................................................... 2600/10280
.................................................................................................... 2700/10280
---
..........................i...............i......................................................... 5200/10280
.................................................................................................... 5300/10280
..........................................................................i......................... 5400/10280
....................................................................i............................... 5500/10280
....................................................................................ii.ii........i.. 5600/10280
...........................i........................................................................ 5800/10280
...................................i................................................................ 5900/10280
.........................................................................................ii......... 6000/10280
............................i....................................................................... 6100/10280
............................i....................................................................... 6100/10280
.................................................................................................... 6200/10280
.................................................................................................... 6300/10280
...................................................ii...i..ii...........i........................... 6400/10280
.................................................................................................... 6600/10280
.................................................................................................... 6700/10280
.................................................................................................... 6700/10280
....................................................................................i..ii........... 6800/10280
.................................................................................................... 7000/10280
.................................................................................................... 7100/10280
......................................i............................................................. 7200/10280
.................................................................................................... 7300/10280
---
...........F........................................................................................ 8200/10280
.................................................................................................... 8300/10280
.............................................................................i...................... 8400/10280
.................................................................................................... 8500/10280
..............................iiiiii.iiiiii.i...............FF...................................... 8600/10280
.................................................................................................... 8800/10280
.................................................................................................... 8900/10280
.................................................................................................... 9000/10280
.................................................................................................... 9100/10280
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097/issue-62097.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issues/issue-62097.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62097.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:12:31
   |
LL |     pub async fn run_dummy_fn(&self) { //~ ERROR cannot infer
   |                               |
   |                               this data with an anonymous lifetime `'_`...
   |                               ...is captured here...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL |         foo(|| self.bar()).await;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0758`.
---
188 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound/must_outlive_least_region_or_bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/must_outlive_least_region_or_bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:3:35
   |
LL | fn elided(x: &i32) -> impl Copy { x }
   |              ----                 ^ ...is captured here...
   |              this data with an anonymous lifetime `'_`...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL | fn elided(x: &i32) -> impl Copy { x }
   |                       ^^^^^^^^^
help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'_` lifetime bound
   |
LL | fn elided(x: &i32) -> impl Copy + '_ { x }

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:6:44
   |
   |
LL | fn explicit<'a>(x: &'a i32) -> impl Copy { x }
   |                    -------                 ^ ...is captured here...
   |                    this data with lifetime `'a`...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL | fn explicit<'a>(x: &'a i32) -> impl Copy { x }
   |                                ^^^^^^^^^
help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'a` lifetime bound
   |
LL | fn explicit<'a>(x: &'a i32) -> impl Copy + 'a { x }

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:9:46
   |
   |
LL | fn elided2(x: &i32) -> impl Copy + 'static { x }
   |               ----                           ^ ...is captured here...
   |               this data with an anonymous lifetime `'_`...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL | fn elided2(x: &i32) -> impl Copy + 'static { x }
   |                        ^^^^^^^^^^^^^^^^^^^
help: consider changing the `impl Trait`'s explicit `'static` bound to argument `x`
   |
LL | fn elided2(x: &i32) -> impl Copy + '_ { x }
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn elided2(x: &'static i32) -> impl Copy + 'static { x }

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:12:55
   |
   |
LL | fn explicit2<'a>(x: &'a i32) -> impl Copy + 'static { x }
   |                     -------                           ^ ...is captured here...
   |                     this data with lifetime `'a`...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL | fn explicit2<'a>(x: &'a i32) -> impl Copy + 'static { x }
   |                                 ^^^^^^^^^^^^^^^^^^^
help: consider changing the `impl Trait`'s explicit `'static` bound to argument `x`
   |
LL | fn explicit2<'a>(x: &'a i32) -> impl Copy + 'a { x }
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn explicit2<'a>(x: &'static i32) -> impl Copy + 'static { x }

error[E0621]: explicit lifetime required in the type of `x`
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:15:24
   |
   |
LL | fn foo<'a>(x: &i32) -> impl Copy + 'a { x }
   |               ----     ^^^^^^^^^^^^^^ lifetime `'a` required
   |               |
   |               help: add explicit lifetime `'a` to the type of `x`: `&'a i32`
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:33:69
   |
   |
LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }
   |                      ------- this data with lifetime `'a`...        ^ ...is captured here...
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider changing the `impl Trait`'s explicit `'static` bound to argument `x`
   |
LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'a { x }
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn with_bound<'a>(x: &'static i32) -> impl LifetimeTrait<'a> + 'static { x }

error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:38:61
   |
---

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:43:51
   |
LL | fn ty_param_wont_outlive_static<T:Debug>(x: T) -> impl Debug + 'static {
   |                                 --                ^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
   |                                 |
   |                                 help: consider adding an explicit lifetime bound...: `T: 'static +`
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:18:50
   |
   |
LL | fn elided3(x: &i32) -> Box<dyn Debug> { Box::new(x) }
   |               ----                               ^ ...is captured here, requiring it to live as long as `'static`
   |               this data with an anonymous lifetime `'_`...
   |
   |
help: to declare that the trait object captures data from argument `x`, you can add an explicit `'_` lifetime bound
   |
LL | fn elided3(x: &i32) -> Box<dyn Debug + '_> { Box::new(x) }

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:21:59
   |
   |
LL | fn explicit3<'a>(x: &'a i32) -> Box<dyn Debug> { Box::new(x) }
   |                     -------                               ^ ...is captured here, requiring it to live as long as `'static`
   |                     this data with lifetime `'a`...
   |
help: to declare that the trait object captures data from argument `x`, you can add an explicit `'a` lifetime bound
   |
   |
LL | fn explicit3<'a>(x: &'a i32) -> Box<dyn Debug + 'a> { Box::new(x) }

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:24:60
   |
   |
LL | fn elided4(x: &i32) -> Box<dyn Debug + 'static> { Box::new(x) }
   |               ----                                         ^ ...is captured here, requiring it to live as long as `'static`
   |               this data with an anonymous lifetime `'_`...
   |
   |
help: consider changing the trait object's explicit `'static` bound to argument `x`
   |
LL | fn elided4(x: &i32) -> Box<dyn Debug + '_> { Box::new(x) }
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn elided4(x: &'static i32) -> Box<dyn Debug + 'static> { Box::new(x) }

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:27:69
   |
   |
LL | fn explicit4<'a>(x: &'a i32) -> Box<dyn Debug + 'static> { Box::new(x) }
   |                     ------- this data with lifetime `'a`...         ^ ...is captured here, requiring it to live as long as `'static`
   |
help: consider changing the trait object's explicit `'static` bound to argument `x`
   |
LL | fn explicit4<'a>(x: &'a i32) -> Box<dyn Debug + 'a> { Box::new(x) }
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn explicit4<'a>(x: &'static i32) -> Box<dyn Debug + 'static> { Box::new(x) }

error: aborting due to 12 previous errors

Some errors have detailed explanations: E0310, E0621, E0623, E0758.
---
43 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered/static-return-lifetime-infered.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/static-return-lifetime-infered.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/static-return-lifetime-infered/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:7:16
   |
LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> {
   |                         ----- this data with an anonymous lifetime `'_`...
LL |         self.x.iter().map(|a| a.0)
   |         ------ ^^^^
   |         ...is captured here...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^
help: to declare that the `impl Trait` captures data from argument `self`, you can add an explicit `'_` lifetime bound
   |
LL |     fn iter_values_anon(&self) -> impl Iterator<Item=u32> + '_ {

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/impl-trait/static-return-lifetime-infered.rs:11:16
   |
   |
LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> {
   |                        -------- this data with lifetime `'a`...
LL |         self.x.iter().map(|a| a.0)
   |         ------ ^^^^
   |         ...is captured here...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> {
help: to declare that the `impl Trait` captures data from argument `self`, you can add an explicit `'a` lifetime bound
   |
   |
LL |     fn iter_values<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0758`.
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16922/issue-16922.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-16922.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16922.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16922" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16922/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/issues/issue-16922.rs:4:14
   |
LL | fn foo<T: Any>(value: &T) -> Box<dyn Any> {
   |                       -- this data with an anonymous lifetime `'_`...
LL |     Box::new(value) as Box<dyn Any>
   |              ^^^^^ ...is captured here, requiring it to live as long as `'static`
   |
help: to declare that the trait object captures data from argument `value`, you can add an explicit `'_` lifetime bound
   |
LL | fn foo<T: Any>(value: &T) -> Box<dyn Any + '_> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0758`.
---
27 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error/object-lifetime-default-from-box-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-from-box-error.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs:18:5
   |
LL | fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait> {
   |             --------------- this data with an anonymous lifetime `'_`...
...
LL |     ss.r //~ ERROR cannot infer an appropriate lifetime
   |     ^^^^ ...is captured and required live as long as `'static` here
   |
help: to declare that the trait object captures data from argument `ss`, you can add an explicit `'_` lifetime bound
   |
LL | fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait + '_> {

error[E0621]: explicit lifetime required in the type of `ss`
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs:31:12
   |
   |
LL | fn store1<'b>(ss: &mut SomeStruct, b: Box<dyn SomeTrait+'b>) {
   |                   --------------- help: add explicit lifetime `'b` to the type of `ss`: `&mut SomeStruct<'b>`
...
LL |     ss.r = b; //~ ERROR explicit lifetime required in the type of `ss` [E0621]
   |            ^ lifetime `'b` required
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0621, E0758.
For more information about an error, try `rustc --explain E0621`.
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion/region-object-lifetime-in-coercion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/region-object-lifetime-in-coercion.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:8:46
   |
LL | fn a(v: &[u8]) -> Box<dyn Foo + 'static> {
   |         ----- this data with an anonymous lifetime `'_`...
LL |     let x: Box<dyn Foo + 'static> = Box::new(v); //~ ERROR cannot infer an appropriate lifetime
   |                                              ^ ...is captured here, requiring it to live as long as `'static`
   |
help: consider changing the trait object's explicit `'static` bound to argument `v`
   |
LL | fn a(v: &[u8]) -> Box<dyn Foo + '_> {
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn a(v: &'static [u8]) -> Box<dyn Foo + 'static> {

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:13:14
   |
   |
LL | fn b(v: &[u8]) -> Box<dyn Foo + 'static> {
   |         ----- this data with an anonymous lifetime `'_`...
LL |     Box::new(v) //~ ERROR cannot infer an appropriate lifetime
   |              ^ ...is captured here, requiring it to live as long as `'static`
   |
help: consider changing the trait object's explicit `'static` bound to argument `v`
   |
LL | fn b(v: &[u8]) -> Box<dyn Foo + '_> {
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn b(v: &'static [u8]) -> Box<dyn Foo + 'static> {

error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:19:14
   |
   |
LL | fn c(v: &[u8]) -> Box<dyn Foo> {
   |         ----- this data with an anonymous lifetime `'_`...
...
LL |     Box::new(v) //~ ERROR cannot infer an appropriate lifetime
   |              ^ ...is captured here, requiring it to live as long as `'static`
   |
help: to declare that the trait object captures data from argument `v`, you can add an explicit `'_` lifetime bound
   |
LL | fn c(v: &[u8]) -> Box<dyn Foo + '_> {

error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:23:14
   |
   |
LL |     Box::new(v) //~ ERROR cannot infer an appropriate lifetime due to conflicting
   |
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined on the function body at 22:6...
   |
   |
LL | fn d<'a,'b>(v: &'a [u8]) -> Box<dyn Foo+'b> {
note: ...so that the expression is assignable
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:23:14
   |
   |
LL |     Box::new(v) //~ ERROR cannot infer an appropriate lifetime due to conflicting
   = note: expected `&[u8]`
              found `&'a [u8]`
              found `&'a [u8]`
note: but, the lifetime must be valid for the lifetime `'b` as defined on the function body at 22:9...
   |
   |
LL | fn d<'a,'b>(v: &'a [u8]) -> Box<dyn Foo+'b> {
note: ...so that the expression is assignable
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:23:5
   |
   |
LL |     Box::new(v) //~ ERROR cannot infer an appropriate lifetime due to conflicting
   |     ^^^^^^^^^^^
   = note: expected `std::boxed::Box<(dyn Foo + 'b)>`
              found `std::boxed::Box<dyn Foo>`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0495, E0758.
For more information about an error, try `rustc --explain E0495`.
---
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2/regions-close-object-into-object-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-2.rs:10:11
   |
LL | fn g<'a, T: 'static>(v: Box<dyn A<T> + 'a>) -> Box<dyn X + 'static> {
   |                         ------------------ this data with lifetime `'a`...
LL |     box B(&*v) as Box<dyn X> //~ ERROR cannot infer
   |           ^^^ ...is captured here, requiring it to live as long as `'static`
   |
help: consider changing the trait object's explicit `'static` bound to argument `v`
   |
LL | fn g<'a, T: 'static>(v: Box<dyn A<T> + 'a>) -> Box<dyn X + 'a> {
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn g<'a, T: 'static>(v: std::boxed::Box<(dyn A<T> + 'static)>) -> Box<dyn X + 'static> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0758`.
---
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4/regions-close-object-into-object-4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-4.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-4.rs:10:11
   |
LL | fn i<'a, T, U>(v: Box<dyn A<U>+'a>) -> Box<dyn X + 'static> {
   |                   ---------------- this data with lifetime `'a`...
LL |     box B(&*v) as Box<dyn X> //~ ERROR cannot infer
   |           ^^^ ...is captured here, requiring it to live as long as `'static`
   |
help: consider changing the trait object's explicit `'static` bound to argument `v`
   |
LL | fn i<'a, T, U>(v: Box<dyn A<U>+'a>) -> Box<dyn X + 'a> {
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn i<'a, T, U>(v: std::boxed::Box<(dyn A<U> + 'static)>) -> Box<dyn X + 'static> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0758`.
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-proc-bound-capture/regions-proc-bound-capture.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-proc-bound-capture.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-proc-bound-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-proc-bound-capture" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-proc-bound-capture/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/regions/regions-proc-bound-capture.rs:9:14
   |
LL | fn static_proc(x: &isize) -> Box<dyn FnMut() -> (isize) + 'static> {
   |                   ------ this data with an anonymous lifetime `'_`...
LL |     // This is illegal, because the region bound on `proc` is 'static.
LL |     Box::new(move || { *x }) //~ ERROR cannot infer an appropriate lifetime
   |              ^^^^^^^^^^^^^^ ...is captured here, requiring it to live as long as `'static`
   |
help: consider changing the trait object's explicit `'static` bound to argument `x`
   |
LL | fn static_proc(x: &isize) -> Box<dyn FnMut() -> (isize) + '_> {
help: alternatively, add an explicit `'static` bound to this reference
   |
   |
LL | fn static_proc(x: &'static isize) -> Box<dyn FnMut() -> (isize) + 'static> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0758`.
---
17 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async/arbitrary_self_types_pin_lifetime_impl_trait-async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs:8:16
   |
LL |     async fn f(self: Pin<&Self>) -> impl Clone { self }
   |                ^^^^  ---------- this data with an anonymous lifetime `'_`...
   |                ...is captured here...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL |     async fn f(self: Pin<&Self>) -> impl Clone { self }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0758`.
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait/arbitrary_self_types_pin_lifetime_impl_trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs:6:44
   |
LL |     fn f(self: Pin<&Self>) -> impl Clone { self } //~ ERROR cannot infer an appropriate lifetime
   |                ----------                  ^^^^ ...is captured here...
   |                this data with an anonymous lifetime `'_`...
   |
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL |     fn f(self: Pin<&Self>) -> impl Clone { self } //~ ERROR cannot infer an appropriate lifetime
   |                               ^^^^^^^^^^
help: to declare that the `impl Trait` captures data from argument `self`, you can add an explicit `'_` lifetime bound
   |
LL |     fn f(self: Pin<&Self>) -> impl Clone + '_ { self } //~ ERROR cannot infer an appropriate lifetime

error: aborting due to previous error

For more information about this error, try `rustc --explain E0758`.
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature/missing-lifetimes-in-signature.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:37:11
   |
LL | fn baz<G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ //~ ERROR undeclared lifetime
   |        -  ^^ undeclared lifetime
   |        |
   |        help: consider introducing lifetime `'a` here: `'a,`
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:19:5
   |
   |
LL |   fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce()
   |                            ------ this data with an anonymous lifetime `'_`...
LL | /     move || { //~ ERROR cannot infer an appropriate lifetime
LL | /     move || { //~ ERROR cannot infer an appropriate lifetime
LL | |         *dest = g.get();
LL | |     }
   | |_____^ ...is captured here...
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce()
   |                                     ^^^^^^^^^^^^^
help: to declare that the `impl Trait` captures data from argument `dest`, you can add an explicit `'_` lifetime bound
   |
LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_

error[E0311]: the parameter type `G` may not live long enough
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:25:37
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime #1 defined on the function body at 25:1...
   |
   |
LL | / fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
LL | | //~^ ERROR the parameter type `G` may not live long enough
LL | | where
LL | |     G: Get<T>
LL | |     }
LL | | }
   | |_^
   | |_^
note: ...so that the type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:30:5: 32:6 g:G, dest:&mut T]` will meet its required lifetime bounds
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
help: consider introducing an explicit lifetime bound
   |
   |
LL | fn bar<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a

error[E0311]: the parameter type `G` may not live long enough
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:47:45
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime #1 defined on the function body at 47:1...
   |
   |
LL | / fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
LL | | //~^ ERROR the parameter type `G` may not live long enough
LL | | where
LL | |     G: Get<T>
LL | |     }
LL | | }
   | |_^
   | |_^
note: ...so that the type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:52:5: 54:6 g:G, dest:&mut T]` will meet its required lifetime bounds
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
help: consider introducing an explicit lifetime bound
   |
   |
LL | fn qux<'b, 'a, G: 'b + 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'b

error[E0311]: the parameter type `G` may not live long enough
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:59:58
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime #1 defined on the method body at 59:5...
   |
   |
LL | /     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
LL | |         //~^ ERROR the parameter type `G` may not live long enough
LL | |         move || {
LL | |             *dest = g.get();
LL | |     }
   | |_____^
   | |_____^
note: ...so that the type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:61:9: 63:10 g:G, dest:&mut T]` will meet its required lifetime bounds
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
help: consider introducing an explicit lifetime bound
   |
   |
LL |     fn qux<'c, 'b, G: 'c + Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'c {

error[E0621]: explicit lifetime required in the type of `dest`
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:68:45
   |
   |
LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
   |                                  ------     ^^^^^^^^^^^^^^^^^^^^^^^ lifetime `'a` required
   |                                  |
   |                                  help: add explicit lifetime `'a` to the type of `dest`: `&'a mut T`
error[E0309]: the parameter type `G` may not live long enough
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:79:44
   |
   |
LL | fn bak<'a, G, T>(g: G, dest: &'a mut T) -> impl FnOnce() + 'a
   |            -                               ^^^^^^^^^^^^^^^^^^ ...so that the type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:84:5: 86:6 g:G, dest:&mut T]` will meet its required lifetime bounds
   |            |
   |            help: consider adding an explicit lifetime bound...: `G: 'a`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0261, E0309, E0621, E0758.
For more information about an error, try `rustc --explain E0261`.
---
17 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore/dyn-trait-underscore.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args underscore-lifetime/dyn-trait-underscore.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/dyn-trait-underscore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/dyn-trait-underscore/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0758]: cannot infer an appropriate lifetime
  --> /checkout/src/test/ui/underscore-lifetime/dyn-trait-underscore.rs:8:20
   |
LL | fn a<T>(items: &[T]) -> Box<dyn Iterator<Item=&T>> {
   |                ---- this data with an anonymous lifetime `'_`...
LL |     //                      ^^^^^^^^^^^^^^^^^^^^^ bound *here* defaults to `'static`
LL |     Box::new(items.iter()) //~ ERROR cannot infer an appropriate lifetime
   |     ---------------^^^^--- ...is captured and required live as long as `'static` here
   |
help: to declare that the trait object captures data from argument `items`, you can add an explicit `'_` lifetime bound
   |
LL | fn a<T>(items: &[T]) -> Box<dyn Iterator<Item=&T> + '_> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0758`.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:07:37
Build completed unsuccessfully in 1:07:37
== clock drift check ==
  local time: Tue Jun  2 19:02:45 UTC 2020
  network time: Tue, 02 Jun 2020 19:02:45 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72804/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72804/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3740) (python)
##[section]Finishing: Finalize Job
