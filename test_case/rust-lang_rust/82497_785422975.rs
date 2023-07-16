plain
    Finished release [optimized] target(s) in 25.18s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 11491 tests
......................................................F.................................F........... 100/11491
.................................................................................................... 300/11491
.................................................................................................... 400/11491
.................................................................................................... 500/11491
.................................................................................................... 600/11491
---
................................................iiiii............................................... 2900/11491
.................................................................................................... 3000/11491
.................................................................................................... 3100/11491
.................................................................................................... 3200/11491
............F..F..............................................F..................................... 3300/11491
.................................................................................................... 3500/11491
.................................................................................................... 3500/11491
............................F......................................................F................ 3600/11491
.................................................................................................... 3800/11491
.................................................................................................... 3900/11491
.................................................................................................... 4000/11491
................................................................F................................... 4100/11491
---
.................................................................................................... 6100/11491
.................................................................................................... 6200/11491
................................ii.ii.......i...i................................................... 6300/11491
....................................................................i...i........................... 6400/11491
...F.i.........................i............F..FF.................F....................FF.....F..... 6500/11491
.........................F....F..........FF........i..................F............................F 6600/11491
.....................................ii............................................i................ 6800/11491
.................................................................................................... 6900/11491
.................................................................................................... 7000/11491
.................................................................i.................................. 7100/11491
---
.................................................................................................... 9200/11491
.................................................................................................... 9300/11491
.................................................................................................... 9400/11491
...............................................i......i............................................. 9500/11491
......................................................................................iiiiiii..iiiii 9600/11491
.................................................................................................... 9800/11491
.................................................................................................... 9900/11491
.................................................................................................... 10000/11491
.................................................................................................... 10100/11491
---
.................................................................................................... 10800/11491
.......................................i............................................................ 10900/11491
.................................................................................................... 11000/11491
.................................................................................................... 11100/11491
..................................................................................F................. 11200/11491
..........................................................................F...FF.FFFFFF............. 11300/11491
...........................................................................................
failures:

---- [ui] ui/annotate-snippet/missing-type.rs stdout ----
---
-    |
- LL |     let x: Iter;
-    |            ^^^^ not found in this scope
-    |
+ {"message":"`--error-format=human-annotate-rs` is unstable","code":null,"level":"error","spans":[],"children":[],"rendered":"error: `--error-format=human-annotate-rs` is unstable\n\n"}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/missing-type/missing-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/missing-type/missing-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args annotate-snippet/missing-type.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/annotate-snippet/missing-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/missing-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "human-annotate-rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/missing-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--error-format=human-annotate-rs` is unstable

------------------------------------------



---- [ui] ui/annotate-snippet/multispan.rs stdout ----
diff of stderr:

- error: hello to you, too!
-   --> $DIR/multispan.rs:15:5
-    |
- LL |     hello!(hi);
-    |
-    |
- error: hello to you, too!
-   --> $DIR/multispan.rs:18:5
-    |
- LL |     hello!(hi hi);
-    |
-    |
- error: hello to you, too!
-   --> $DIR/multispan.rs:21:5
-    |
- LL |     hello!(hi hi hi);
-    |
-    |
- error: hello to you, too!
-   --> $DIR/multispan.rs:24:5
-    |
- LL |     hello!(hi hey hi yo hi beep beep hi hi);
-    |
-    |
- error: hello to you, too!
-   --> $DIR/multispan.rs:25:5
-    |
- LL |     hello!(hi there, hi how are you? hi... hi.);
-    |
-    |
- error: hello to you, too!
-   --> $DIR/multispan.rs:26:5
-    |
- LL |     hello!(whoah. hi di hi di ho);
-    |
-    |
- error: hello to you, too!
-   --> $DIR/multispan.rs:27:5
-    |
- LL |     hello!(hi good hi and good bye);
-    |
-    |
+ {"message":"`--error-format=human-annotate-rs` is unstable","code":null,"level":"error","spans":[],"children":[],"rendered":"error: `--error-format=human-annotate-rs` is unstable\n\n"}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/multispan/multispan.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/multispan/multispan.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args annotate-snippet/multispan.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/annotate-snippet/multispan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/multispan" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "human-annotate-rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/multispan/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--error-format=human-annotate-rs` is unstable

------------------------------------------



---- [ui] ui/deduplicate-diagnostics.rs#duplicate stdout ----
diff of stderr:

22 LL | #[deny("literal")]
23    |        ^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/deduplicate-diagnostics.rs:8:8
-    |
-    |
- LL | #[deny("literal")]
-    |        ^^^^^^^^^ bad attribute argument
- error: aborting due to 5 previous errors
+ error: aborting due to 4 previous errors
32 
33 For more information about this error, try `rustc --explain E0452`.
33 For more information about this error, try `rustc --explain E0452`.
34 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics.duplicate/deduplicate-diagnostics.duplicate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args deduplicate-diagnostics.rs`

error in revision `duplicate`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deduplicate-diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "duplicate" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics.duplicate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics.duplicate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/deduplicate-diagnostics.rs:8:8
   |
LL | #[deny("literal")] //~ ERROR malformed lint attribute input
   |        ^^^^^^^^^ bad attribute argument

error: cannot find derive macro `Unresolved` in this scope
  --> /checkout/src/test/ui/deduplicate-diagnostics.rs:4:10
   |
LL | #[derive(Unresolved)] //~ ERROR cannot find derive macro `Unresolved` in this scope


error: cannot find derive macro `Unresolved` in this scope
  --> /checkout/src/test/ui/deduplicate-diagnostics.rs:4:10
   |
LL | #[derive(Unresolved)] //~ ERROR cannot find derive macro `Unresolved` in this scope

error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/deduplicate-diagnostics.rs:8:8
   |
   |
LL | #[deny("literal")] //~ ERROR malformed lint attribute input
   |        ^^^^^^^^^ bad attribute argument
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0452`.


------------------------------------------


---- [ui] ui/error-codes/E0453.rs stdout ----
diff of stderr:

16 LL | #[allow(non_snake_case)]
17    |         ^^^^^^^^^^^^^^ overruled by previous forbid
18 
- error[E0453]: allow(non_snake_case) incompatible with previous forbid
-   --> $DIR/E0453.rs:3:9
-    |
- LL | #![forbid(non_snake_case)]
-    |           -------------- `forbid` level set here
- LL | 
- LL | #[allow(non_snake_case)]
-    |         ^^^^^^^^^^^^^^ overruled by previous forbid
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
29 
30 For more information about this error, try `rustc --explain E0453`.
30 For more information about this error, try `rustc --explain E0453`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453/E0453.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0453.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0453.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0453]: allow(non_snake_case) incompatible with previous forbid
   |
   |
LL | #![forbid(non_snake_case)]
   |           -------------- `forbid` level set here
LL | 
LL | #[allow(non_snake_case)]
   |         ^^^^^^^^^^^^^^ overruled by previous forbid

error[E0453]: allow(non_snake_case) incompatible with previous forbid
   |
   |
LL | #![forbid(non_snake_case)]
   |           -------------- `forbid` level set here
LL | 
LL | #[allow(non_snake_case)]
   |         ^^^^^^^^^^^^^^ overruled by previous forbid
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0453`.


------------------------------------------


---- [ui] ui/error-codes/E0452.rs stdout ----
diff of stderr:

22 LL | #![allow(foo = "")]
23    |          ^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/E0452.rs:1:10
-    |
-    |
- LL | #![allow(foo = "")]
-    |          ^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/E0452.rs:1:10
-    |
-    |
- LL | #![allow(foo = "")]
-    |          ^^^^^^^^ bad attribute argument
- error: aborting due to 6 previous errors
+ error: aborting due to 4 previous errors
38 
39 For more information about this error, try `rustc --explain E0452`.
39 For more information about this error, try `rustc --explain E0452`.
40 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0452/E0452.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0452.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0452.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0452" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0452/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/error-codes/E0452.rs:1:10
   |
LL | #![allow(foo = "")] //~ ERROR E0452
   |          ^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/error-codes/E0452.rs:1:10
   |
   |
LL | #![allow(foo = "")] //~ ERROR E0452
   |          ^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/error-codes/E0452.rs:1:10
   |
   |
LL | #![allow(foo = "")] //~ ERROR E0452
   |          ^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/error-codes/E0452.rs:1:10
   |
   |
LL | #![allow(foo = "")] //~ ERROR E0452
   |          ^^^^^^^^ bad attribute argument
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0452`.


------------------------------------------


---- [ui] ui/error-codes/E0602.rs stdout ----
diff of stderr:

6    |
7    = note: requested on the command line with `-D bogus`
8 
- error[E0602]: unknown lint: `bogus`
-    |
-    = note: requested on the command line with `-D bogus`
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
14 
15 For more information about this error, try `rustc --explain E0602`.
15 For more information about this error, try `rustc --explain E0602`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0602/E0602.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0602.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0602.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0602" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "bogus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0602/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0602]: unknown lint: `bogus`
   |
   = note: requested on the command line with `-D bogus`

error[E0602]: unknown lint: `bogus`
   |
   = note: requested on the command line with `-D bogus`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0602`.

---
18 
- error[E0658]: lint reasons are experimental
-   --> $DIR/feature-gate-lint-reasons.rs:1:28
-    |
- LL | #![warn(nonstandard_style, reason = "the standard should be respected")]
-    |
-    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
-    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable
- 
---
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-lint-reasons/feature-gate-lint-reasons.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-lint-reasons.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-lint-reasons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-lint-reasons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-lint-reasons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: lint reasons are experimental
  --> /checkout/src/test/ui/feature-gates/feature-gate-lint-reasons.rs:1:28
   |
LL | #![warn(nonstandard_style, reason = "the standard should be respected")]
   |
   = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
   = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: lint reasons are experimental
  --> /checkout/src/test/ui/feature-gates/feature-gate-lint-reasons.rs:1:28
   |
LL | #![warn(nonstandard_style, reason = "the standard should be respected")]
   |
   = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
   = help: add `#![feature(lint_reasons)]` to the crate attributes to enable

---
---- [ui] ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs stdout ----
diff of stderr:

16    = note: see issue #71668 <https://github.com/rust-lang/rust/issues/71668> for more information
17    = help: add `#![feature(unsafe_block_in_unsafe_fn)]` to the crate attributes to enable
18 
- error[E0658]: the `unsafe_op_in_unsafe_fn` lint is unstable
-   --> $DIR/feature-gate-unsafe_block_in_unsafe_fn.rs:1:1
-    |
- LL | #![deny(unsafe_op_in_unsafe_fn)]
-    |
-    = note: see issue #71668 <https://github.com/rust-lang/rust/issues/71668> for more information
-    = note: see issue #71668 <https://github.com/rust-lang/rust/issues/71668> for more information
-    = help: add `#![feature(unsafe_block_in_unsafe_fn)]` to the crate attributes to enable
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
29 
30 For more information about this error, try `rustc --explain E0658`.
30 For more information about this error, try `rustc --explain E0658`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn/feature-gate-unsafe_block_in_unsafe_fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `unsafe_op_in_unsafe_fn` lint is unstable
   |
   |
LL | #![deny(unsafe_op_in_unsafe_fn)]
   |
   = note: see issue #71668 <https://github.com/rust-lang/rust/issues/71668> for more information
   = note: see issue #71668 <https://github.com/rust-lang/rust/issues/71668> for more information
   = help: add `#![feature(unsafe_block_in_unsafe_fn)]` to the crate attributes to enable

error[E0658]: the `unsafe_op_in_unsafe_fn` lint is unstable
   |
   |
LL | #![deny(unsafe_op_in_unsafe_fn)]
   |
   = note: see issue #71668 <https://github.com/rust-lang/rust/issues/71668> for more information
   = note: see issue #71668 <https://github.com/rust-lang/rust/issues/71668> for more information
   = help: add `#![feature(unsafe_block_in_unsafe_fn)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/hygiene/no_implicit_prelude-2021.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/no_implicit_prelude-2021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude-2021" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude-2021/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: edition 2021 is unstable and only available with -Z unstable-options.

------------------------------------------



---- [ui] ui/lint/crate_level_only_lint.rs stdout ----
diff of stderr:

40 LL | #[allow(uncommon_codepoints)]
42 
42 
- error: allow(uncommon_codepoints) is ignored unless specified at crate level
-   --> $DIR/crate_level_only_lint.rs:4:10
-    |
- LL | #![allow(uncommon_codepoints)]
- 
- 
- error: allow(uncommon_codepoints) is ignored unless specified at crate level
-   --> $DIR/crate_level_only_lint.rs:9:9
-    |
- LL | #[allow(uncommon_codepoints)]
- 
- 
- error: allow(uncommon_codepoints) is ignored unless specified at crate level
-   --> $DIR/crate_level_only_lint.rs:17:9
-    |
- LL | #[allow(uncommon_codepoints)]
- 
- error: aborting due to 9 previous errors
+ error: aborting due to 6 previous errors
62 
62 
63 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/crate_level_only_lint/crate_level_only_lint.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/crate_level_only_lint.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/crate_level_only_lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/crate_level_only_lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/crate_level_only_lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: allow(uncommon_codepoints) is ignored unless specified at crate level
   |
   |
LL | #![allow(uncommon_codepoints)]
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/crate_level_only_lint.rs:1:30
   |
   |
LL | #![deny(uncommon_codepoints, unused_attributes)]


error: allow(uncommon_codepoints) is ignored unless specified at crate level
   |
   |
LL | #[allow(uncommon_codepoints)]


error: allow(uncommon_codepoints) is ignored unless specified at crate level
   |
   |
LL | #[allow(uncommon_codepoints)]


error: allow(uncommon_codepoints) is ignored unless specified at crate level
   |
   |
LL | #![allow(uncommon_codepoints)]


error: allow(uncommon_codepoints) is ignored unless specified at crate level
   |
   |
LL | #[allow(uncommon_codepoints)]


error: allow(uncommon_codepoints) is ignored unless specified at crate level
   |
   |
LL | #[allow(uncommon_codepoints)]

error: aborting due to 6 previous errors


---

75    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
76    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
77 
- error: allow(nonstandard_style) incompatible with previous forbid
-   --> $DIR/forbid-group-group-2.rs:7:9
-    |
- LL | #![forbid(warnings)]
-    |           -------- `forbid` level set here
- ...
- LL | #[allow(nonstandard_style)]
-    |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
- 
- 
- error: allow(nonstandard_style) incompatible with previous forbid
-   --> $DIR/forbid-group-group-2.rs:7:9
-    |
- LL | #![forbid(warnings)]
-    |           -------- `forbid` level set here
- ...
- LL | #[allow(nonstandard_style)]
-    |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
- 
- 
- error: allow(nonstandard_style) incompatible with previous forbid
-   --> $DIR/forbid-group-group-2.rs:7:9
-    |
- LL | #![forbid(warnings)]
-    |           -------- `forbid` level set here
- ...
- LL | #[allow(nonstandard_style)]
-    |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
- 
- error: aborting due to 9 previous errors
- error: aborting due to 9 previous errors
+ error: aborting due to 6 previous errors
115 
116 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-group-2/forbid-group-group-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/forbid-group-group-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/forbid-group-group-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-group-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-group-2/auxiliary"
------------------------------------------

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: allow(nonstandard_style) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
...
LL | #[allow(nonstandard_style)]
   |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/forbid-group-group-2.rs:5:9
   |
   |
LL | #![deny(forbidden_lint_groups)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


error: allow(nonstandard_style) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
...
LL | #[allow(nonstandard_style)]
   |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


error: allow(nonstandard_style) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
...
LL | #[allow(nonstandard_style)]
   |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


error: allow(nonstandard_style) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
...
LL | #[allow(nonstandard_style)]
   |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


error: allow(nonstandard_style) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
...
LL | #[allow(nonstandard_style)]
   |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


error: allow(nonstandard_style) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
...
LL | #[allow(nonstandard_style)]
   |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>

error: aborting due to 6 previous errors
---

---- [ui] ui/lint/forbid-member-group.rs stdout ----
diff of stderr:

16 LL | #[allow(unused)]
17    |         ^^^^^^ overruled by previous forbid
18 
- error[E0453]: allow(unused) incompatible with previous forbid
-   --> $DIR/forbid-member-group.rs:6:9
-    |
- LL | #![forbid(unused_variables)]
-    |           ---------------- `forbid` level set here
- LL | 
- LL | #[allow(unused)]
-    |         ^^^^^^ overruled by previous forbid
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
29 
30 For more information about this error, try `rustc --explain E0453`.
30 For more information about this error, try `rustc --explain E0453`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-member-group/forbid-member-group.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/forbid-member-group.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/forbid-member-group.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-member-group" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-member-group/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0453]: allow(unused) incompatible with previous forbid
   |
   |
LL | #![forbid(unused_variables)]
   |           ---------------- `forbid` level set here
LL | 
LL | #[allow(unused)]
   |         ^^^^^^ overruled by previous forbid

error[E0453]: allow(unused) incompatible with previous forbid
   |
   |
LL | #![forbid(unused_variables)]
   |           ---------------- `forbid` level set here
LL | 
LL | #[allow(unused)]
   |         ^^^^^^ overruled by previous forbid
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0453`.

---

35    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
36    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
37 
- warning: allow(unused_variables) incompatible with previous forbid
-   --> $DIR/forbid-group-member.rs:8:9
-    |
- LL | #![forbid(unused)]
-    |           ------ `forbid` level set here
- LL | 
- LL | #[allow(unused_variables)]
-    |         ^^^^^^^^^^^^^^^^ overruled by previous forbid
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
- 
- warning: 4 warnings emitted
- warning: 4 warnings emitted
+ warning: 3 warnings emitted
51 
52 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-member/forbid-group-member.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/forbid-group-member.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/forbid-group-member.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-member" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-member/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: allow(unused_variables) incompatible with previous forbid
   |
   |
LL | #![forbid(unused)]
   |           ------ `forbid` level set here
LL | 
LL | #[allow(unused_variables)]
   |         ^^^^^^^^^^^^^^^^ overruled by previous forbid
   |
   = note: `#[warn(forbidden_lint_groups)]` on by default
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


warning: allow(unused_variables) incompatible with previous forbid
   |
   |
LL | #![forbid(unused)]
   |           ------ `forbid` level set here
LL | 
LL | #[allow(unused_variables)]
   |         ^^^^^^^^^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


warning: allow(unused_variables) incompatible with previous forbid
   |
   |
LL | #![forbid(unused)]
   |           ------ `forbid` level set here
LL | 
LL | #[allow(unused_variables)]
   |         ^^^^^^^^^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>

warning: 3 warnings emitted
---

35    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
36    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
37 
- warning: deny(warnings) incompatible with previous forbid
-   --> $DIR/issue-80988.rs:7:8
-    |
- LL | #![forbid(warnings)]
-    |           -------- `forbid` level set here
- LL | 
- LL | #[deny(warnings)]
-    |        ^^^^^^^^ overruled by previous forbid
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
- 
- warning: 4 warnings emitted
- warning: 4 warnings emitted
+ warning: 3 warnings emitted
51 
52 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-80988/issue-80988.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/issue-80988.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-80988.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-80988" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-80988/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: deny(warnings) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
LL | #[deny(warnings)]
LL | #[deny(warnings)]
   |        ^^^^^^^^ overruled by previous forbid
   |
   = note: `#[warn(forbidden_lint_groups)]` on by default
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


warning: deny(warnings) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
LL | #[deny(warnings)]
LL | #[deny(warnings)]
   |        ^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>


warning: deny(warnings) incompatible with previous forbid
   |
   |
LL | #![forbid(warnings)]
   |           -------- `forbid` level set here
LL | #[deny(warnings)]
LL | #[deny(warnings)]
   |        ^^^^^^^^ overruled by previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>

warning: 3 warnings emitted
---

---- [ui] ui/lint/lint-forbid-attr.rs stdout ----
diff of stderr:

16 LL | #[allow(deprecated)]
17    |         ^^^^^^^^^^ overruled by previous forbid
18 
- error[E0453]: allow(deprecated) incompatible with previous forbid
-   --> $DIR/lint-forbid-attr.rs:3:9
-    |
- LL | #![forbid(deprecated)]
-    |           ---------- `forbid` level set here
- LL | 
- LL | #[allow(deprecated)]
-    |         ^^^^^^^^^^ overruled by previous forbid
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
29 
30 For more information about this error, try `rustc --explain E0453`.
30 For more information about this error, try `rustc --explain E0453`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr/lint-forbid-attr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-forbid-attr.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-forbid-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0453]: allow(deprecated) incompatible with previous forbid
   |
   |
LL | #![forbid(deprecated)]
   |           ---------- `forbid` level set here
LL | 
LL | #[allow(deprecated)]
   |         ^^^^^^^^^^ overruled by previous forbid

error[E0453]: allow(deprecated) incompatible with previous forbid
   |
   |
LL | #![forbid(deprecated)]
   |           ---------- `forbid` level set here
LL | 
LL | #[allow(deprecated)]
   |         ^^^^^^^^^^ overruled by previous forbid
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0453`.


------------------------------------------


---- [ui] ui/lint/lint-forbid-cmdline.rs stdout ----
diff of stderr:

14    |
15    = note: `forbid` lint level was set on command line
16 
- error[E0453]: allow(deprecated) incompatible with previous forbid
-   --> $DIR/lint-forbid-cmdline.rs:3:9
-    |
- LL | #[allow(deprecated)]
-    |         ^^^^^^^^^^ overruled by previous forbid
-    |
-    = note: `forbid` lint level was set on command line
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
26 
27 For more information about this error, try `rustc --explain E0453`.
27 For more information about this error, try `rustc --explain E0453`.
28 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline/lint-forbid-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-forbid-cmdline.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "deprecated" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0453]: allow(deprecated) incompatible with previous forbid
   |
   |
LL | #[allow(deprecated)] //~ ERROR allow(deprecated) incompatible
   |         ^^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

error[E0453]: allow(deprecated) incompatible with previous forbid
   |
   |
LL | #[allow(deprecated)] //~ ERROR allow(deprecated) incompatible
   |         ^^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0453`.


------------------------------------------


---- [ui] ui/lint/lint-malformed.rs stdout ----
diff of stderr:

28 LL | #![allow(bar = "baz")]
29    |          ^^^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/lint-malformed.rs:2:10
-    |
-    |
- LL | #![allow(bar = "baz")]
-    |          ^^^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/lint-malformed.rs:2:10
-    |
-    |
- LL | #![allow(bar = "baz")]
-    |          ^^^^^^^^^^^ bad attribute argument
- error: aborting due to 7 previous errors
+ error: aborting due to 5 previous errors
44 
45 For more information about this error, try `rustc --explain E0452`.
45 For more information about this error, try `rustc --explain E0452`.
46 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-malformed/lint-malformed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-malformed.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-malformed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-malformed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-malformed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/lint-malformed.rs:2:10
   |
LL | #![allow(bar = "baz")] //~ ERROR malformed lint attribute
   |          ^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/lint-malformed.rs:2:10
   |
   |
LL | #![allow(bar = "baz")] //~ ERROR malformed lint attribute
   |          ^^^^^^^^^^^ bad attribute argument

error: malformed `deny` attribute input
   |
   |
LL | #![deny = "foo"] //~ ERROR malformed `deny` attribute input
   | ^^^^^^^^^^^^^^^^ help: must be of the form: `#[deny(lint1, lint2, ..., /*opt*/ reason = "...")]`
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/lint-malformed.rs:2:10
   |
   |
LL | #![allow(bar = "baz")] //~ ERROR malformed lint attribute
   |          ^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/lint-malformed.rs:2:10
   |
   |
LL | #![allow(bar = "baz")] //~ ERROR malformed lint attribute
   |          ^^^^^^^^^^^ bad attribute argument
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0452`.


------------------------------------------


---- [ui] ui/lint/lint-removed-cmdline.rs stdout ----
diff of stderr:

10    |
11    = note: requested on the command line with `-D raw_pointer_derive`
12 
- warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
-    |
-    = note: requested on the command line with `-D raw_pointer_derive`
- 
17 error: unused variable: `unused`
19    |

27    |        ^^^^^^^^
28    = note: `#[deny(unused_variables)]` implied by `#[deny(warnings)]`
---
32 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed-cmdline/lint-removed-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-removed-cmdline.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-removed-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "raw_pointer_derive" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
   |
   = note: requested on the command line with `-D raw_pointer_derive`

warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
   |
   = note: requested on the command line with `-D raw_pointer_derive`

warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
   |
   = note: requested on the command line with `-D raw_pointer_derive`

error: unused variable: `unused`
   |
   |
LL | fn main() { let unused = (); }
   |                 ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-removed-cmdline.rs:11:8
   |
LL | #[deny(warnings)]
---
---- [ui] ui/lint/lint-renamed-cmdline.rs stdout ----
diff of stderr:

10    |
11    = note: requested on the command line with `-D bare_trait_object`
12 
- warning: lint `bare_trait_object` has been renamed to `bare_trait_objects`
-    |
-    = note: requested on the command line with `-D bare_trait_object`
- 
17 error: unused variable: `unused`
19    |

27    |        ^^^^^^
27    |        ^^^^^^
28    = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
- error: aborting due to previous error; 4 warnings emitted
+ error: aborting due to previous error; 3 warnings emitted
31 
32 
32 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-renamed-cmdline/lint-renamed-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-renamed-cmdline.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-renamed-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-renamed-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "bare_trait_object" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-renamed-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `bare_trait_object` has been renamed to `bare_trait_objects`
   |
   = note: requested on the command line with `-D bare_trait_object`

warning: lint `bare_trait_object` has been renamed to `bare_trait_objects`
   |
   = note: requested on the command line with `-D bare_trait_object`

warning: lint `bare_trait_object` has been renamed to `bare_trait_objects`
   |
   = note: requested on the command line with `-D bare_trait_object`

error: unused variable: `unused`
   |
   |
LL | fn main() { let unused = (); }
   |                 ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-renamed-cmdline.rs:7:8
   |
   |
LL | #[deny(unused)]
   |        ^^^^^^
   = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
error: aborting due to previous error; 3 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/lint-unexported-no-mangle.rs stdout ----
diff of stderr:

22    |
23    = note: requested on the command line with `-F private_no_mangle_statics`
24 
- warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
-    |
-    = note: requested on the command line with `-F private_no_mangle_fns`
- 
- warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
-    |
-    = note: requested on the command line with `-F private_no_mangle_statics`
- 
33 error: const items should never be `#[no_mangle]`
35    |

48    | |
48    | |
49    | help: try a static value: `pub static`
- error: aborting due to 2 previous errors; 8 warnings emitted
+ error: aborting due to 2 previous errors; 6 warnings emitted
52 
53 
53 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle/lint-unexported-no-mangle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-unexported-no-mangle.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unexported-no-mangle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "private_no_mangle_fns" "-F" "no_mangle_const_items" "-F" "private_no_mangle_statics" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

error: const items should never be `#[no_mangle]`
   |
   |
LL | const FOO: u64 = 1; //~ ERROR const items should never be `#[no_mangle]`
   | |
   | |
   | help: try a static value: `pub static`
   |
   = note: requested on the command line with `-F no-mangle-const-items`

error: const items should never be `#[no_mangle]`
   |
   |
LL | pub const PUB_FOO: u64 = 1; //~ ERROR const items should never be `#[no_mangle]`
   | |
   | |
   | help: try a static value: `pub static`
error: aborting due to 2 previous errors; 6 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/lint-unknown-lint-cmdline.rs stdout ----
diff of stderr:

16    = help: did you mean: `dead_code`
17    = note: requested on the command line with `-D dead_cod`
18 
- error[E0602]: unknown lint: `bogus`
-    |
-    = note: requested on the command line with `-D bogus`
- 
- error[E0602]: unknown lint: `dead_cod`
-    |
-    = help: did you mean: `dead_code`
-    = note: requested on the command line with `-D dead_cod`
- error: aborting due to 6 previous errors
+ error: aborting due to 4 previous errors
29 
30 For more information about this error, try `rustc --explain E0602`.
30 For more information about this error, try `rustc --explain E0602`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unknown-lint-cmdline/lint-unknown-lint-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-unknown-lint-cmdline.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unknown-lint-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unknown-lint-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "bogus" "-D" "dead_cod" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unknown-lint-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0602]: unknown lint: `bogus`
   |
   = note: requested on the command line with `-D bogus`

error[E0602]: unknown lint: `dead_cod`
   |
   = help: did you mean: `dead_code`
   = note: requested on the command line with `-D dead_cod`

error[E0602]: unknown lint: `bogus`
   |
   = note: requested on the command line with `-D bogus`

error[E0602]: unknown lint: `dead_cod`
   |
   = help: did you mean: `dead_code`
   = note: requested on the command line with `-D dead_cod`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0602`.


------------------------------------------


---- [ui] ui/lint/reasons-erroneous.rs stdout ----
diff of stderr:

126 LL | #![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
127    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:3:58
-    |
-    |
- LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
-    |                                                          ^ reason must be a string literal
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:10:40
-    |
-    |
- LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
-    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:17:29
-    |
-    |
- LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
-    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:17:29
-    |
-    |
- LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
-    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:30:23
-    |
-    |
- LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
-    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:30:23
-    |
-    |
- LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
-    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:43:36
-    |
-    |
- LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
-    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:43:36
-    |
-    |
- LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
-    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:56:44
-    |
-    |
- LL | #![warn(ellipsis_inclusive_range_patterns, reason = "born barren", reason = "a freak growth")]
-    |                                            ^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
- error[E0452]: malformed lint attribute input
-   --> $DIR/reasons-erroneous.rs:63:25
-    |
-    |
- LL | #![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
-    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
- error: aborting due to 30 previous errors; 1 warning emitted
+ error: aborting due to 20 previous errors; 1 warning emitted
190 
191 For more information about this error, try `rustc --explain E0452`.
191 For more information about this error, try `rustc --explain E0452`.
192 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous/reasons-erroneous.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/reasons-erroneous.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/reasons-erroneous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:3:58
   |
LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
   |                                                          ^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:10:40
   |
   |
LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:56:44
   |
   |
LL | #![warn(ellipsis_inclusive_range_patterns, reason = "born barren", reason = "a freak growth")]
   |                                            ^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:63:25
   |
   |
LL | #![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last

warning: unknown lint: `reason`
   |
   |
LL | #![warn(missing_copy_implementations, reason)]
   |
   = note: `#[warn(unknown_lints)]` on by default

error[E0452]: malformed lint attribute input
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:3:58
   |
LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
   |                                                          ^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:10:40
   |
   |
LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:56:44
   |
   |
LL | #![warn(ellipsis_inclusive_range_patterns, reason = "born barren", reason = "a freak growth")]
   |                                            ^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:63:25
   |
   |
LL | #![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error: aborting due to 20 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0452`.


------------------------------------------


---- [ui] ui/lint/use_suggestion_json.rs stdout ----
diff of stderr:

- {
-   "message": "cannot find type `Iter` in this scope",
-   "code": {
-     "code": "E0412",
-     "explanation": "A used type name is not in scope.
+ {"message":"`--error-format=pretty-json` is unstable","code":null,"level":"error","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1m\u001b[38;5;9merror\u001b[0m\u001b[0m\u001b[1m: `--error-format=pretty-json` is unstable\u001b[0m
- Erroneous code examples:
- 
- 