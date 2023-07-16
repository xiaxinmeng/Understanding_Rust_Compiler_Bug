plain
.................................................................................................... 2200/11715
.................................................................................................... 2300/11715
.................................................................................................... 2400/11715
.................................................................................................... 2500/11715
..........................................F..F..........F..F........................................ 2600/11715
.................................................................................................... 2800/11715
...................................................................................................i 2900/11715
iiii................................................................................................ 3000/11715
.................................................................................................... 3100/11715
---
.................................................................................................... 9400/11715
.................................................................................................... 9500/11715
........................................................i......i.................................... 9600/11715
.................................................................................................... 9700/11715
..iiiiiii..iiiiii.i................................................................................. 9800/11715
.................................................................................................... 10000/11715
.................................................................................................... 10100/11715
.................................................................................................... 10200/11715
.................................................................................................... 10300/11715
---

9 
10 warning: 1 warning emitted
11 
+ Future incompatibility report: Future breakage date: None, diagnostic:
+ warning: detects usage of old versions of or patterns
+   --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
+    |
+ LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
+    |
+    |
+    = note: `#[allow(or_patterns_back_compat)]` on by default
+    = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
+    = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
+    = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.
12 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/run_pass/lit-pattern-matching-with-methods/lit-pattern-matching-with-methods.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/run_pass/lit-pattern-matching-with-methods.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/run_pass/lit-pattern-matching-with-methods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/run_pass/lit-pattern-matching-with-methods" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/run_pass/lit-pattern-matching-with-methods/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(capture_disjoint_fields)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information


warning: 1 warning emitted

Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.

------------------------------------------



---- [ui] ui/command/command-current-dir.rs stdout ----
normalized stderr:
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
  --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/command-current-dir/command-current-dir.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args command/command-current-dir.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/command/command-current-dir.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/command-current-dir/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/command-current-dir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.

------------------------------------------



---- [ui] ui/consts/issue-73976-polymorphic.rs stdout ----
diff of stderr:

24 
25 error: aborting due to 4 previous errors
26 
+ Future incompatibility report: Future breakage date: None, diagnostic:
+ warning: detects usage of old versions of or patterns
+   --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
+    |
+ LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
+    |
+    |
+    = note: `#[allow(or_patterns_back_compat)]` on by default
+    = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
+    = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
+    = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.
27 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-polymorphic/issue-73976-polymorphic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-73976-polymorphic.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-73976-polymorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-polymorphic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-polymorphic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: constant pattern depends on a generic parameter
   |
   |
LL |     matches!(GetTypeId::<T>::VALUE, GetTypeId::<T>::VALUE)


error: constant pattern depends on a generic parameter
   |
   |
LL |     matches!(GetTypeNameLen::<T>::VALUE, GetTypeNameLen::<T>::VALUE)


error: constant pattern depends on a generic parameter
   |
   |
LL |     matches!(GetTypeId::<T>::VALUE, GetTypeId::<T>::VALUE)


error: constant pattern depends on a generic parameter
   |
   |
LL |     matches!(GetTypeNameLen::<T>::VALUE, GetTypeNameLen::<T>::VALUE)

error: aborting due to 4 previous errors


Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.

------------------------------------------



---- [ui] ui/consts/issue-73976-monomorphic.rs stdout ----
normalized stderr:
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
  --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-monomorphic/issue-73976-monomorphic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-73976-monomorphic.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-73976-monomorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-monomorphic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-monomorphic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.

------------------------------------------



---- [ui] ui/consts/issue-79137-toogeneric.rs stdout ----
diff of stderr:

12 
13 error: aborting due to 2 previous errors
14 
+ Future incompatibility report: Future breakage date: None, diagnostic:
+ warning: detects usage of old versions of or patterns
+   --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
+    |
+ LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
+    |
+    |
+    = note: `#[allow(or_patterns_back_compat)]` on by default
+    = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
+    = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
+    = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.
15 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-toogeneric/issue-79137-toogeneric.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-79137-toogeneric.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-79137-toogeneric.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-toogeneric" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-toogeneric/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: constant pattern depends on a generic parameter
   |
   |
LL |     matches!(GetVariantCount::<T>::VALUE, GetVariantCount::<T>::VALUE)


error: constant pattern depends on a generic parameter
   |
   |
LL |     matches!(GetVariantCount::<T>::VALUE, GetVariantCount::<T>::VALUE)

error: aborting due to 2 previous errors


Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.

------------------------------------------



---- [ui] ui/consts/issue-79137-monomorphic.rs stdout ----
normalized stderr:
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
  --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-monomorphic/issue-79137-monomorphic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-79137-monomorphic.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-79137-monomorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-monomorphic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-monomorphic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.

------------------------------------------



---- [ui] ui/expr/compound-assignment/eval-order.rs stdout ----
normalized stderr:
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
  --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/compound-assignment/eval-order/eval-order.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args expr/compound-assignment/eval-order.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/expr/compound-assignment/eval-order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/compound-assignment/eval-order/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/compound-assignment/eval-order/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.

------------------------------------------



---- [ui] ui/macros/assert-matches-macro-msg.rs stdout ----
normalized stderr:
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
  --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
   |
LL |     ($left:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => ({
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.
Future breakage date: None, diagnostic:
Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
  --> $SRC_DIR/core/src/macros/mod.rs:LL:COL
   |
LL |     ($left:expr, $( $pattern:pat )|+ $( if $guard: expr )?, $($arg:tt)+) => ({
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-matches-macro-msg/assert-matches-macro-msg.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/assert-matches-macro-msg.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/assert-matches-macro-msg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-matches-macro-msg/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-matches-macro-msg/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($left:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => ({
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.
Future breakage date: None, diagnostic:
Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($left:expr, $( $pattern:pat )|+ $( if $guard: expr )?, $($arg:tt)+) => ({
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.

------------------------------------------



---- [ui] ui/macros/macro-follow-rpass.rs stdout ----
normalized stderr:
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($p:pat |) => {};
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: in Rust 2021, the pat matcher will match new patterns, which include the | character. Please use pat2015 to avoid breakage.



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-follow-rpass/macro-follow-rpass.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-follow-rpass.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-follow-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-follow-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-follow-rpass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Future incompatibility report: Future breakage date: None, diagnostic:
warning: detects usage of old versions of or patterns
   |
   |
LL |     ($p:pat |) => {};
   |
   |
   = note: `#[allow(or_patterns_back_compat)]` on by default
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
   = note: for more information, see issue #83318 <https://github.com/rust-lang/rust/issues/83318>
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:37
