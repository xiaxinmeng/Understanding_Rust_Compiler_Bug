plain
running 11508 tests
.................................................................................................... 100/11508
.......................................i........ii.................................................. 200/11508
.................................................................................................... 300/11508
................................................................................F..F................ 400/11508
.................................................................................................... 600/11508
..............i.........................................................................ii.......... 700/11508
.................................................................................................... 800/11508
.................................................................................................... 900/11508
---
....................................................iiiii........................................... 2900/11508
.................................................................................................... 3000/11508
.................................................................................................... 3100/11508
.................................................................................................... 3200/11508
............F....F.................................................................................. 3300/11508
.................................................................................................... 3500/11508
.................................................................................................... 3500/11508
................................F......................................................F............ 3600/11508
.................................................................................................... 3800/11508
.................................................................................................... 3900/11508
.................................................................................................... 4000/11508
.................................................................................................... 4100/11508
---
....i............................................................................................... 6100/11508
.................................................................................................... 6200/11508
.....................................ii.ii.......i...i.............................................. 6300/11508
..........................................................................i....i.................... 6400/11508
...........i....F.....................i..............FF.F..............F......................F.F... 6500/11508
.F........................................................i..................F...................... 6600/11508
.............................................ii............................................i........ 6800/11508
.................................................................................................... 6900/11508
.................................................................................................... 7000/11508
.........................................................................i.......................... 7100/11508
---
.....................................................................i.............................. 8300/11508
.................................................................................................... 8400/11508
.................................................................................................... 8500/11508
.................................................................................................... 8600/11508
.................................................F.F................................................ 8700/11508
....................i..............i................................................................ 8900/11508
.................................................................................................... 9000/11508
.................................................................................................... 9100/11508
.................................................................................................... 9200/11508
---
.......................................................i............................................ 10900/11508
...........................................................F........................................ 11000/11508
.................................................................................................... 11100/11508
.................................................................................................... 11200/11508
.....F............................................................................................F. 11300/11508
...................................................................................................i 11400/11508
........
failures:

---- [ui] ui/associated-types/defaults-cyclic-fail-1.rs stdout ----
---- [ui] ui/associated-types/defaults-cyclic-fail-1.rs stdout ----
diff of stderr:

4 LL |     type A = Box<Self::B>;
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0275]: overflow evaluating the requirement `<usize as Tr>::A == _`
+    |
+    |
+ LL |     type B = &'static Self::A;
+ 
+ error: aborting due to 2 previous errors
8 
9 For more information about this error, try `rustc --explain E0275`.
9 For more information about this error, try `rustc --explain E0275`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1/defaults-cyclic-fail-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/defaults-cyclic-fail-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-cyclic-fail-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `<bool as Tr>::B == _`
   |
   |
LL |     type A = Box<Self::B>;


error[E0275]: overflow evaluating the requirement `<usize as Tr>::A == _`
   |
   |
LL |     type B = &'static Self::A;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0275`.
For more information about this error, try `rustc --explain E0275`.

------------------------------------------


---- [ui] ui/associated-types/defaults-cyclic-fail-2.rs stdout ----
diff of stderr:

4 LL |     type A = Box<Self::B>;
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0275]: overflow evaluating the requirement `<usize as Tr>::A == _`
+    |
+    |
+ LL |     type B = &'static Self::A;
+ 
+ error: aborting due to 2 previous errors
8 
9 For more information about this error, try `rustc --explain E0275`.
9 For more information about this error, try `rustc --explain E0275`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2/defaults-cyclic-fail-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/defaults-cyclic-fail-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-cyclic-fail-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `<bool as Tr>::B == _`
   |
   |
LL |     type A = Box<Self::B>;


error[E0275]: overflow evaluating the requirement `<usize as Tr>::A == _`
   |
   |
LL |     type B = &'static Self::A;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0275`.
For more information about this error, try `rustc --explain E0275`.

------------------------------------------


---- [ui] ui/deduplicate-diagnostics.rs#duplicate stdout ----

error in revision `duplicate`: /checkout/src/test/ui/deduplicate-diagnostics.rs:8: expected error not found: malformed lint attribute input

error in revision `duplicate`: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deduplicate-diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "duplicate" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics.duplicate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics.duplicate/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute input",
]


thread '[ui] ui/deduplicate-diagnostics.rs#duplicate' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---- [ui] ui/error-codes/E0452.rs stdout ----

error: /checkout/src/test/ui/error-codes/E0452.rs:1: expected error not found: E0452
error: /checkout/src/test/ui/error-codes/E0452.rs:1: expected error not found: E0452

error: /checkout/src/test/ui/error-codes/E0452.rs:1: expected error not found: E0452

error: 0 unexpected errors found, 2 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0452.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0452" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0452/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
---
thread '[ui] ui/error-codes/E0452.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---- [ui] ui/error-codes/E0453.rs stdout ----

error: /checkout/src/test/ui/error-codes/E0453.rs:3: expected error not found: allow(non_snake_case) incompatible
error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0453.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453/auxiliary"
    Error {
        line_num: 3,
        kind: Some(
            Error,
            Error,
        ),
        msg: "allow(non_snake_case) incompatible",
]

thread '[ui] ui/error-codes/E0453.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/feature-gates/feature-gate-lint-reasons.rs stdout ----

error: /checkout/src/test/ui/feature-gates/feature-gate-lint-reasons.rs:1: expected error not found: lint reasons are experimental

error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-lint-reasons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-lint-reasons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-lint-reasons/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
---
thread '[ui] ui/feature-gates/feature-gate-lint-reasons.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---- [ui] ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs stdout ----

error: /checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs:1: expected error not found: the `unsafe_op_in_unsafe_fn` lint is unstable
error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the `unsafe_op_in_unsafe_fn` lint is unstable",
]

thread '[ui] ui/feature-gates/feature-gate-unsafe_block_in_unsafe_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/lint/crate_level_only_lint.rs stdout ----

error: /checkout/src/test/ui/lint/crate_level_only_lint.rs:4: expected error not found: allow(uncommon_codepoints) is ignored unless specified at crate level [unused_attributes]

error: /checkout/src/test/ui/lint/crate_level_only_lint.rs:9: expected error not found: allow(uncommon_codepoints) is ignored unless specified at crate level [unused_attributes]

error: /checkout/src/test/ui/lint/crate_level_only_lint.rs:17: expected error not found: allow(uncommon_codepoints) is ignored unless specified at crate level [unused_attributes]
error: 0 unexpected errors found, 3 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/crate_level_only_lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/crate_level_only_lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/crate_level_only_lint/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "allow(uncommon_codepoints) is ignored unless specified at crate level [unused_attributes]",
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "allow(uncommon_codepoints) is ignored unless specified at crate level [unused_attributes]",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "allow(uncommon_codepoints) is ignored unless specified at crate level [unused_attributes]",
]

thread '[ui] ui/lint/crate_level_only_lint.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---
error: /checkout/src/test/ui/lint/forbid-group-group-2.rs:7: expected warning not found: previously accepted by the compiler

error: 0 unexpected errors found, 6 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/forbid-group-group-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-group-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-group-2/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Error,
---
error: /checkout/src/test/ui/lint/forbid-member-group.rs:6: expected error not found: incompatible with previous forbid

error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/forbid-member-group.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-member-group" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-member-group/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "incompatible with previous forbid",
]

thread '[ui] ui/lint/forbid-member-group.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---
error: /checkout/src/test/ui/lint/forbid-group-member.rs:8: expected warning not found: previously accepted

error: 0 unexpected errors found, 2 expected errors not found
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/forbid-group-member.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-member" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-member/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "incompatible with previous forbid",
    Error {
        line_num: 8,
        kind: Some(
            Warning,
---
error: /checkout/src/test/ui/lint/issue-80988.rs:7: expected warning not found: being phased out

error: 0 unexpected errors found, 2 expected errors not found
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-80988.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-80988" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-80988/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "incompatible with previous forbid",
    Error {
        line_num: 7,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "being phased out",
]

thread '[ui] ui/lint/issue-80988.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/lint/lint-forbid-attr.rs stdout ----

error: /checkout/src/test/ui/lint/lint-forbid-attr.rs:3: expected error not found: allow(deprecated) incompatible
error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-forbid-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr/auxiliary"
    Error {
        line_num: 3,
        kind: Some(
            Error,
            Error,
        ),
        msg: "allow(deprecated) incompatible",
]

thread '[ui] ui/lint/lint-forbid-attr.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/lint/lint-forbid-cmdline.rs stdout ----

error: /checkout/src/test/ui/lint/lint-forbid-cmdline.rs:3: expected error not found: allow(deprecated) incompatible
error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "deprecated" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline/auxiliary"
    Error {
        line_num: 3,
        kind: Some(
            Error,
            Error,
        ),
        msg: "allow(deprecated) incompatible",
]

thread '[ui] ui/lint/lint-forbid-cmdline.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---
error: /checkout/src/test/ui/lint/lint-malformed.rs:2: expected error not found: malformed lint attribute

error: 0 unexpected errors found, 2 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-malformed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-malformed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-malformed/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
]

thread '[ui] ui/lint/lint-malformed.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---
error: /checkout/src/test/ui/lint/reasons-erroneous.rs:17: expected error not found: malformed lint attribute

error: /checkout/src/test/ui/lint/reasons-erroneous.rs:17: expected error not found: malformed lint attribute

error: /checkout/src/test/ui/lint/reasons-erroneous.rs:17: expected note not found: bad attribute argument

error: /checkout/src/test/ui/lint/reasons-erroneous.rs:17: expected note not found: bad attribute argument
error: /checkout/src/test/ui/lint/reasons-erroneous.rs:30: expected error not found: malformed lint attribute

error: /checkout/src/test/ui/lint/reasons-erroneous.rs:30: expected error not found: malformed lint attribute


error: /checkout/src/test/ui/lint/reasons-erroneous.rs:30: expected note not found: bad attribute argument

error: /checkout/src/test/ui/lint/reasons-erroneous.rs:30: expected note not found: bad attribute argument
error: /checkout/src/test/ui/lint/reasons-erroneous.rs:43: expected error not found: malformed lint attribute

error: /checkout/src/test/ui/lint/reasons-erroneous.rs:43: expected error not found: malformed lint attribute


error: /checkout/src/test/ui/lint/reasons-erroneous.rs:43: expected note not found: bad attribute argument

error: /checkout/src/test/ui/lint/reasons-erroneous.rs:43: expected note not found: bad attribute argument
error: /checkout/src/test/ui/lint/reasons-erroneous.rs:56: expected error not found: malformed lint attribute


error: /checkout/src/test/ui/lint/reasons-erroneous.rs:56: expected note not found: reason in lint attribute must come last
error: /checkout/src/test/ui/lint/reasons-erroneous.rs:63: expected error not found: malformed lint attribute


error: /checkout/src/test/ui/lint/reasons-erroneous.rs:63: expected note not found: reason in lint attribute must come last
error: 0 unexpected errors found, 20 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/reasons-erroneous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous/auxiliary"
    Error {
        line_num: 3,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 3,
        kind: Some(
            Note,
            Note,
        ),
        msg: "reason must be a string literal",
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 10,
        kind: Some(
            Note,
            Note,
        ),
        msg: "reason must be a string literal",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 17,
        kind: Some(
            Note,
            Note,
        ),
        msg: "bad attribute argument",
    Error {
        line_num: 17,
        kind: Some(
            Note,
            Note,
        ),
        msg: "bad attribute argument",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 30,
        kind: Some(
            Note,
            Note,
        ),
        msg: "bad attribute argument",
    Error {
        line_num: 30,
        kind: Some(
            Note,
            Note,
        ),
        msg: "bad attribute argument",
    Error {
        line_num: 43,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 43,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 43,
        kind: Some(
            Note,
            Note,
        ),
        msg: "bad attribute argument",
    Error {
        line_num: 43,
        kind: Some(
            Note,
            Note,
        ),
        msg: "bad attribute argument",
    Error {
        line_num: 56,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 56,
        kind: Some(
            Note,
            Note,
        ),
        msg: "reason in lint attribute must come last",
    Error {
        line_num: 63,
        kind: Some(
            Error,
            Error,
        ),
        msg: "malformed lint attribute",
    Error {
        line_num: 63,
        kind: Some(
            Note,
            Note,
        ),
        msg: "reason in lint attribute must come last",
]

thread '[ui] ui/lint/reasons-erroneous.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/privacy/privacy2.rs stdout ----
diff of stderr:

23 
24 error: requires `sized` lang_item
25 
- error: requires `sized` lang_item
- 
- error: requires `sized` lang_item
- error: aborting due to 5 previous errors
+ error: aborting due to 3 previous errors
31 
32 Some errors have detailed explanations: E0432, E0603.
32 Some errors have detailed explanations: E0432, E0603.
33 For more information about an error, try `rustc --explain E0432`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/privacy2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/privacy2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `bar::foo`
   |
LL |     use bar::foo;
LL |     use bar::foo;
   |         ^^^^^^^^ no `foo` in `bar`
error[E0603]: function import `foo` is private
  --> /checkout/src/test/ui/privacy/privacy2.rs:23:20
   |
   |
LL |     use bar::glob::foo;
   |                    ^^^ private function import
   |
note: the function import `foo` is defined here...
   |
LL |         use foo;
   |             ^^^
   |             ^^^
note: ...and refers to the function `foo` which is defined here
   |
LL | pub fn foo() {}
   | ^^^^^^^^^^^^ consider importing it directly


error: requires `sized` lang_item
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0432, E0603.
For more information about an error, try `rustc --explain E0432`.
---
---- [ui] ui/privacy/privacy3.rs stdout ----
diff of stderr:

6 
7 error: requires `sized` lang_item
8 
- error: requires `sized` lang_item
- 
- error: requires `sized` lang_item
- error: aborting due to 4 previous errors
+ error: aborting due to 2 previous errors
14 
15 For more information about this error, try `rustc --explain E0432`.
15 For more information about this error, try `rustc --explain E0432`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/privacy3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/privacy3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `bar::gpriv`
   |
   |
LL |     use bar::gpriv;
   |         ^^^^^^^^^^ no `gpriv` in `bar`

error: requires `sized` lang_item
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.


------------------------------------------


---- [ui] ui/tool_lints.rs stdout ----

error: /checkout/src/test/ui/tool_lints.rs:1: expected error not found: an unknown tool name found in scoped lint: `foo::bar`
error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tool_lints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "an unknown tool name found in scoped lint: `foo::bar`",
]

thread '[ui] ui/tool_lints.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/type_length_limit.rs stdout ----
diff of stderr:

7    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
8    = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
9 
- error: reached the type-length limit while instantiating `std::rt::lang_start::<()>::{closure#0}`
-   --> $SRC_DIR/std/src/rt.rs:LL:COL
-    |
- LL |         &move || crate::sys_common::backtrace::__rust_begin_short_backtrace(main).report(),
-    |
-    |
-    = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
19 
20 
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type_length_limit.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: reached the type-length limit while instantiating `std::mem::drop::<Option<((((...,....., ...), ..., ...), ..., ...)>>`
   |
   |
LL | pub fn drop<T>(_x: T) {}
   |
   |
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.long-type.txt'
   = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/unknown-lint-tool-name.rs stdout ----

error: /checkout/src/test/ui/unknown-lint-tool-name.rs:1: expected error not found: an unknown tool name found in scoped lint: `foo::bar`

error: /checkout/src/test/ui/unknown-lint-tool-name.rs:5: expected error not found: an unknown tool name found in scoped lint: `foo::bar`
error: 0 unexpected errors found, 2 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unknown-lint-tool-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unknown-lint-tool-name" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unknown-lint-tool-name/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "an unknown tool name found in scoped lint: `foo::bar`",
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "an unknown tool name found in scoped lint: `foo::bar`",
]

thread '[ui] ui/unknown-lint-tool-name.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/unused-crate-deps/extern-loc-json-json.rs stdout ----
diff of stderr:

- {"message":"external crate `bar` unused in `extern_loc_json_json`: remove the dependency or add `use bar as _;`","code":{"code":"unused_crate_dependencies","explanation":null},"level":"warning","spans":[{"file_name":"$DIR/extern-loc-json-json.rs","byte_start":189,"byte_end":189,"line_start":7,"line_end":7,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![warn(unused_crate_dependencies)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"$DIR/extern-loc-json-json.rs","byte_start":197,"byte_end":222,"line_start":7,"line_end":7,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"#![warn(unused_crate_dependencies)]","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove unnecessary dependency `bar`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"json extern location","code":null,"level":"help","spans":[],"children":[],"rendered":null,"tool_metadata":{"key":123,"value":{}}}],"rendered":"warning: external crate `bar` unused in `extern_loc_json_json`: remove the dependency or add `use bar as _;`
-   --> $DIR/extern-loc-json-json.rs:7:1
+ {"message":"external crate `bar` unused in `extern_loc_json_json`: remove the dependency or add `use bar as _;`","code":{"code":"unused_crate_dependencies","explanation":null},"level":"warning","spans":[{"file_name":"$DIR/extern-loc-json-json.rs","byte_start":215,"byte_end":215,"line_start":8,"line_end":8,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![warn(unused_crate_dependencies)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"$DIR/extern-loc-json-json.rs","byte_start":223,"byte_end":248,"line_start":8,"line_end":8,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"#![warn(unused_crate_dependencies)]","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove unnecessary dependency `bar`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"json extern location","code":null,"level":"help","spans":[],"children":[],"rendered":null,"tool_metadata":{"key":123,"value":{}}}],"rendered":"warning: external crate `bar` unused in `extern_loc_json_json`: remove the dependency or add `use bar as _;`
3    |
4 LL | #![warn(unused_crate_dependencies)]
5    | ^

---
11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json/extern-loc-json-json.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-json-json.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-json-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=json:{\"key\":123,\"value\":{}}" "--error-format" "json" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json/auxiliary/libbar.so" "-Zunstable-options"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: external crate `bar` unused in `extern_loc_json_json`: remove the dependency or add `use bar as _;`
   |
LL | #![warn(unused_crate_dependencies)]
   | ^
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/unused-crate-deps/extern-loc-json-json.rs:8:9
   |
LL | #![warn(unused_crate_dependencies)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: remove unnecessary dependency `bar`
warning: 1 warning emitted


------------------------------------------
---
test result: FAILED. 11393 passed; 22 failed; 93 ignored; 0 measured; 0 filtered out; finished in 136.94s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:08
