plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9c36e6cb-d884-4f18-b4ad-183ddc7ce88c.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71937/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71937/merge:refs/remotes/pull/71937/merge
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
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
......................i............................................................................. 1800/9988
.................................................................................................... 1900/9988
.......................................i............................................................ 2000/9988
.................................................................................................... 2100/9988
.............................iiiii.................................................................. 2200/9988
.................................................................................................... 2400/9988
.................................................................................................... 2500/9988
.................................................................................................... 2600/9988
.................................................................................................... 2700/9988
---
................i...............i................................................................... 5100/9988
.................................................................................................... 5200/9988
..............................................................i..................................... 5300/9988
.....................................................i.............................................. 5400/9988
.........................................................ii.ii........i...i......................... 5500/9988
..................................................................................................i. 5600/9988
....i...................................F........................................................... 5800/9988
........................................ii.....................................i.................... 5900/9988
.................................................................................................... 6000/9988
.................................................................................................... 6100/9988
.................................................................................................... 6100/9988
............................................................................ii...i..ii...........i.. 6200/9988
.................................................................................................... 6400/9988
.................................................................................................... 6500/9988
.................................................................................................... 6600/9988
.................................................................................................... 6600/9988
........i..ii....................................................................................... 6700/9988
.................................................................................................... 6900/9988
.........i.......................................................................................... 7000/9988
.................................................................................................... 7100/9988
...................................................i................................................ 7200/9988
---
.................................................................................................... 7900/9988
.........................F.......................................................................... 8000/9988
.................................................................................................... 8100/9988
...................i................................................................................ 8200/9988
.........................................................................iiiiii.iiiii.i............. 8300/9988
..........................i......................................................................... 8500/9988
.................................................................................................... 8600/9988
.................................................................................................... 8700/9988
.................................................................................................... 8800/9988
---
1 error[E0573]: expected type, found variant `NoResult`
-   --> $DIR/issue-17546.rs:12:17
+   --> $DIR/issue-17546.rs:14:17
3    |
4 LL |     fn new() -> NoResult<MyEnum, String> {

19    |                 ^^^^^^
20 
21 error[E0573]: expected type, found variant `Result`
21 error[E0573]: expected type, found variant `Result`
-   --> $DIR/issue-17546.rs:22:17
+   --> $DIR/issue-17546.rs:24:17
23    |
24 LL |     fn new() -> Result<foo::MyEnum, String> {

37      and 1 other candidate
38 
39 error[E0573]: expected type, found variant `Result`
39 error[E0573]: expected type, found variant `Result`
-   --> $DIR/issue-17546.rs:28:13
+   --> $DIR/issue-17546.rs:30:13
41    |
42 LL | fn new() -> Result<foo::MyEnum, String> {

55      and 1 other candidate
56 
57 error[E0573]: expected type, found variant `NoResult`
57 error[E0573]: expected type, found variant `NoResult`
-   --> $DIR/issue-17546.rs:33:15
+   --> $DIR/issue-17546.rs:35:15
59    |
60 LL | fn newer() -> NoResult<foo::MyEnum, String> {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/issue-17546.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/issue-17546.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17546.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17546.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0573]: expected type, found variant `NoResult`
  --> /checkout/src/test/ui/issues/issue-17546.rs:14:17
   |
LL |     fn new() -> NoResult<MyEnum, String> {
   | 
  ::: /checkout/src/libcore/result.rs:247:1
   |
LL | pub enum Result<T, E> {
---
LL |     fn new() -> foo::MyEnum {
   |                 ^^^^^^^^^^^
help: an enum with a similar name exists
   |
LL |     fn new() -> Result<MyEnum, String> {

error[E0573]: expected type, found variant `Result`
  --> /checkout/src/test/ui/issues/issue-17546.rs:24:17
   |
   |
LL |     fn new() -> Result<foo::MyEnum, String> {
   |
help: possible better candidates are found in other modules, you can import them into scope
   |
LL |     use std::fmt::Result;
---

error[E0573]: expected type, found variant `Result`
  --> /checkout/src/test/ui/issues/issue-17546.rs:30:13
   |
LL | fn new() -> Result<foo::MyEnum, String> {
   |
help: possible better candidates are found in other modules, you can import them into scope
   |
LL | use std::fmt::Result;
---

error[E0573]: expected type, found variant `NoResult`
  --> /checkout/src/test/ui/issues/issue-17546.rs:35:15
   |
LL | fn newer() -> NoResult<foo::MyEnum, String> {
   | 
  ::: /checkout/src/libcore/result.rs:247:1
   |
LL | pub enum Result<T, E> {
LL | pub enum Result<T, E> {
   | --------------------- similarly named enum `Result` defined here
   |
help: try using the variant's enum
   |
LL | fn newer() -> foo::MyEnum {
help: an enum with a similar name exists
   |
   |
LL | fn newer() -> Result<foo::MyEnum, String> {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0573`.
---
362           "is_primary": true,

380     }
381   ],
382   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror[E0412]\u001b[0m\u001b[0m\u001b[1m: cannot find type `Iter` in this scope\u001b[0m
- \u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0m$DIR/use_suggestion_json.rs:12:12\u001b[0m
+ \u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0m$DIR/use_suggestion_json.rs:13:12\u001b[0m
384 \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
385 \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: Iter;\u001b[0m
386 \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9m^^^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9mnot found in this scope\u001b[0m

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json/use_suggestion_json.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json/use_suggestion_json.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/use_suggestion_json.rs`
error: 1 errors occurred comparing output.
failed to decode compiler output as json: line: {
output: {
  "message": "cannot find type `Iter` in this scope",
  "message": "cannot find type `Iter` in this scope",
  "code": {
    "code": "E0412",
    "explanation": "A used type name is not in scope.\n\nErroneous code examples:\n\n