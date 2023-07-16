plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bd5e7176-c0d1-41e2-8b30-319826ce6683.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72988/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72988/merge:refs/remotes/pull/72988/merge
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
............................i...............i....................................................... 5200/10316
.................................................................................................... 5300/10316
............................................................................i....................... 5400/10316
......................................................................i............................. 5500/10316
.......................................................................................ii.ii........ 5600/10316
i...i..................................................................F............................ 5700/10316
.....................................................................i.............................. 5900/10316
......F............................................................................................. 6000/10316
.......................ii.....................................i..................................... 6100/10316
.................................................................................................... 6200/10316
.................................................................................................... 6200/10316
.................................................................................................... 6300/10316
.....................................................................................ii...i..ii..... 6400/10316
.................................................................................................... 6600/10316
.................................................................................................... 6700/10316
.......................................................................................FF........... 6800/10316
.......................................................................................FF........... 6800/10316
..................i..ii............................................................................. 6900/10316
.................................................................................................... 7100/10316
........................................................................i........................... 7200/10316
.................................................................................................... 7300/10316
.................................................................................................... 7400/10316
---
.................................................................................................... 8200/10316
........F..............................F.............F.............FF............................... 8300/10316
.................................................................................................... 8400/10316
...........i........................................................................................ 8500/10316
............................F....................................iiiiii.iiiiii.i.................... 8600/10316
....................i............................................................................... 8800/10316
.................................................................................................... 8900/10316
.................................................................................................... 9000/10316
.................................................................................................... 9100/10316
---
9 LL | use std::collections::HashMap;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1/issue-31997-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derived-errors/issue-31997-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derived-errors/issue-31997-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/did_you_mean/issue-56028-there-is-an-enum-variant.rs stdout ----
diff of stderr:

15 LL | fn setup() -> PutDown { Set }
17      and 3 other candidates
+ help: did you mean one of these?:
+    |
+    |
+ LL | fn setup() -> std::collections::BTreeSet { Set }
+    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | fn setup() -> std::collections::HashSet { Set }
+    |               ^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | fn setup() -> std::fmt::DebugSet { Set }
18 
19 error[E0425]: cannot find value `Set` in this scope
20   --> $DIR/issue-56028-there-is-an-enum-variant.rs:9:21


22 LL | fn setup() -> Set { Set }
24    |
+ help: did you mean one of these?:
+    |
+    |
+ LL | fn setup() -> Set { std::collections::BTreeSet }
+    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | fn setup() -> Set { std::collections::HashSet }
+    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | fn setup() -> Set { std::fmt::DebugSet }
25 help: consider importing one of these items
26    |
26    |
27 LL | use AffixHeart::Set;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant/issue-56028-there-is-an-enum-variant.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant/issue-56028-there-is-an-enum-variant.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/issue-56028-there-is-an-enum-variant.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `Set` in this scope
  --> /checkout/src/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant.rs:9:15
   |
LL | fn setup() -> Set { Set }
   |
   |
help: there is an enum variant `AffixHeart::Set` and 7 others; try using the variant's enum
   |
LL | fn setup() -> AffixHeart { Set }
   |               ^^^^^^^^^^
LL | fn setup() -> CauseToBe { Set }
   |               ^^^^^^^^^
LL | fn setup() -> Determine { Set }
   |               ^^^^^^^^^
LL | fn setup() -> PutDown { Set }
     and 3 other candidates
help: did you mean one of these?:
   |
   |
LL | fn setup() -> std::collections::BTreeSet { Set }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | fn setup() -> std::collections::HashSet { Set }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^
LL | fn setup() -> std::fmt::DebugSet { Set }

error[E0425]: cannot find value `Set` in this scope
  --> /checkout/src/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant.rs:9:21
   |
   |
LL | fn setup() -> Set { Set }
   |
help: did you mean one of these?:
   |
   |
LL | fn setup() -> Set { std::collections::BTreeSet }
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | fn setup() -> Set { std::collections::HashSet }
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^
LL | fn setup() -> Set { std::fmt::DebugSet }
help: consider importing one of these items
   |
   |
LL | use AffixHeart::Set;
   |
LL | use CauseToBe::Set;
LL | use Determine::Set;
   |
LL | use PutDown::Set;
   |
---
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/for-loop/for-loop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/for-loop.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/for-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/for-loop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/for-loop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/issues/issue-17546.rs stdout ----
diff of stderr:

24 LL |     fn new() -> Result<foo::MyEnum, String> {
26    |
+ help: did you mean one of these?:
+    |
+    |
+ LL |     fn new() -> std::sync::BarrierWaitResult<foo::MyEnum, String> {
+    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     fn new() -> std::sync::WaitTimeoutResult<foo::MyEnum, String> {
27 help: consider importing one of these items instead
28    |
29 LL |     use std::fmt::Result;


42 LL | fn new() -> Result<foo::MyEnum, String> {
44    |
+ help: did you mean one of these?:
+    |
+    |
+ LL | fn new() -> std::sync::BarrierWaitResult<foo::MyEnum, String> {
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | fn new() -> std::sync::WaitTimeoutResult<foo::MyEnum, String> {
45 help: consider importing one of these items instead
46    |
47 LL | use std::fmt::Result;



The actual stderr differed from the expected stderr.
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
help: did you mean one of these?:
   |
   |
LL |     fn new() -> std::sync::BarrierWaitResult<foo::MyEnum, String> {
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL |     fn new() -> std::sync::WaitTimeoutResult<foo::MyEnum, String> {
help: consider importing one of these items instead
   |
LL |     use std::fmt::Result;
   |
---

error[E0573]: expected type, found variant `Result`
  --> /checkout/src/test/ui/issues/issue-17546.rs:30:13
   |
LL | fn new() -> Result<foo::MyEnum, String> {
   |
help: did you mean one of these?:
   |
   |
LL | fn new() -> std::sync::BarrierWaitResult<foo::MyEnum, String> {
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | fn new() -> std::sync::WaitTimeoutResult<foo::MyEnum, String> {
help: consider importing one of these items instead
   |
LL | use std::fmt::Result;
   |
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
---- [ui] ui/issues/issue-27815.rs stdout ----
diff of stderr:

9    |
10 LL |     let v = u32 { x: 1 };
+    |
+ help: did you mean `std::num::NonZeroU32`?
+    |
+    |
+ LL |     let v = std::num::NonZeroU32 { x: 1 };
12 
13 error[E0574]: expected struct, variant or union type, found module `A`
14   --> $DIR/issue-27815.rs:7:9


21    |
22 LL |         u32 { x: 1 } => {}
+    |
+ help: did you mean `std::num::NonZeroU32`?
+    |
+    |
+ LL |         std::num::NonZeroU32 { x: 1 } => {}
24 
25 error: aborting due to 4 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27815/issue-27815.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-27815.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27815.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27815" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27815/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0574]: expected struct, variant or union type, found module `A`
  --> /checkout/src/test/ui/issues/issue-27815.rs:4:13
   |
LL |     let u = A { x: 1 }; //~ ERROR expected struct, variant or union type, found module `A`

error[E0574]: expected struct, variant or union type, found builtin type `u32`
  --> /checkout/src/test/ui/issues/issue-27815.rs:5:13
   |
   |
LL |     let v = u32 { x: 1 }; //~ ERROR expected struct, variant or union type, found builtin type `u32`
   |
help: did you mean `std::num::NonZeroU32`?
   |
   |
LL |     let v = std::num::NonZeroU32 { x: 1 }; //~ ERROR expected struct, variant or union type, found builtin type `u32`

error[E0574]: expected struct, variant or union type, found module `A`
  --> /checkout/src/test/ui/issues/issue-27815.rs:7:9
   |
   |
LL |         A { x: 1 } => {}

error[E0574]: expected struct, variant or union type, found builtin type `u32`
  --> /checkout/src/test/ui/issues/issue-27815.rs:9:9
   |
   |
LL |         u32 { x: 1 } => {}
   |
help: did you mean `std::num::NonZeroU32`?
   |
   |
LL |         std::num::NonZeroU32 { x: 1 } => {}

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0574`.
---

12 error[E0412]: cannot find type `cString` in this scope
13   --> $DIR/ffi.rs:6:21
14    |
- LL |   fn test_cstring(_x: cString){}
-    |                       ^^^^^^^ help: a struct with a similar name exists: `String`
+ LL | fn test_cstring(_x: cString){}
+    | 
+    | 
+   ::: $SRC_DIR/liballoc/string.rs:LL:COL
+ LL | pub struct String {
+    | ----------------- similarly named struct `String` defined here
17 
18 error[E0412]: cannot find type `Osstr` in this scope
18 error[E0412]: cannot find type `Osstr` in this scope
19   --> $DIR/ffi.rs:8:19


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/libstd-case-typo/ffi/ffi.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args libstd-case-typo/ffi.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/libstd-case-typo/ffi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/libstd-case-typo/ffi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/libstd-case-typo/ffi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `cStr` in this scope
  --> /checkout/src/test/ui/libstd-case-typo/ffi.rs:4:18
   |
LL | fn test_cstr(_x: cStr){}
   |
help: did you mean `std::ffi::CStr`?
   |
   |
LL | fn test_cstr(_x: std::ffi::CStr){}

error[E0412]: cannot find type `cString` in this scope
  --> /checkout/src/test/ui/libstd-case-typo/ffi.rs:6:21
   |
   |
LL | fn test_cstring(_x: cString){}
   | 
  ::: /checkout/src/liballoc/string.rs:283:1
   |
LL | pub struct String {
LL | pub struct String {
   | ----------------- similarly named struct `String` defined here

error[E0412]: cannot find type `Osstr` in this scope
  --> /checkout/src/test/ui/libstd-case-typo/ffi.rs:8:19
   |
LL | fn test_osstr(_x: Osstr){}
   |
help: did you mean `std::ffi::OsStr`?
   |
   |
LL | fn test_osstr(_x: std::ffi::OsStr){}

error[E0412]: cannot find type `Osstring` in this scope
  --> /checkout/src/test/ui/libstd-case-typo/ffi.rs:10:22
   |
   |
LL | fn test_osstring(_x: Osstring){}
   |
help: did you mean `std::ffi::OsString`?
   |
   |
LL | fn test_osstring(_x: std::ffi::OsString){}

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0412`.
---

94   ],
95   "children": [
96     {
+       "message": "did you mean one of these?:",
+       "code": null,
+       "level": "help",
+       "spans": [
+           "file_name": "$DIR/use_suggestion_json.rs",
+           "byte_start": 560,
+           "byte_end": 564,
+           "line_start": 13,
+           "line_start": 13,
+           "line_end": 13,
+           "column_start": 12,
+           "column_end": 16,
+           "is_primary": true,
+           "text": [
+             {
+               "text": "    let x: Iter;",
+               "highlight_start": 12,
+               "highlight_end": 16
+           ],
+           "label": null,
+           "suggested_replacement": "std::io::BufWriter",
+           "suggestion_applicability": "MaybeIncorrect",
---
+           "column_end": 16,
+           "is_primary": true,
+           "text": [
+             {
+               "text": "    let x: Iter;",
+               "highlight_start": 12,
+               "highlight_end": 16
+           ],
+           "label": null,
+           "suggested_replacement": "std::io::LineWriter",
+           "suggestion_applicability": "MaybeIncorrect",
---
+     },
+     {
97       "message": "consider importing one of these items",
98       "code": null,
99       "level": "help",

385 \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: Iter;\u001b[0m
386 \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9m^^^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9mnot found in this scope\u001b[0m
387 \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
+ \u001b[0m\u001b[1m\u001b[38;5;14mhelp\u001b[0m\u001b[0m: did you mean one of these?:\u001b[0m
+ \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
+ \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: std::io::BufWriter;\u001b[0m
+ \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;14m^^^^^^^^^^^^^^^^^^\u001b[0m
+ \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: std::io::LineWriter;\u001b[0m
+ \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;14m^^^^^^^^^^^^^^^^^^^\u001b[0m
388 \u001b[0m\u001b[1m\u001b[38;5;14mhelp\u001b[0m\u001b[0m: consider importing one of these items\u001b[0m
389 \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
390 \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::binary_heap::Iter;\u001b[0m

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