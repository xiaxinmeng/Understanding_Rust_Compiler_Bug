plain
---- [ui] src/test/ui/never_type/issue-96335.rs stdout ----
diff of stderr:

8    |
9 LL |     0....{loop{}1};
- help: or `..=` for an inclusive range
+ help: use `..=` for an inclusive range
12    |
12    |
13 LL |     0..=..{loop{}1};


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-96335/issue-96335.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-96335/issue-96335.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args never_type/issue-96335.rs`

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/issue-96335.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-96335" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-96335/auxiliary"
stdout: none
--- stderr -------------------------------
error: unexpected token: `...`
   |
   |
LL |     0.....{loop{}1};
   |
help: use `..` for an exclusive range
   |
   |
LL |     0....{loop{}1};
help: use `..=` for an inclusive range
   |
   |
LL |     0..=..{loop{}1};

error[E0308]: mismatched types
  --> /checkout/src/test/ui/never_type/issue-96335.rs:2:9
   |
   |
LL |     0.....{loop{}1};
   |     |   |
   |     |   expected integer, found struct `RangeTo`
   |     arguments to this function are incorrect
   |
   |
   = note: expected type `{integer}`
            found struct `RangeTo<{integer}>`
note: associated function defined here
  --> /checkout/library/core/src/ops/range.rs:374:18
   |
LL |     pub const fn new(start: Idx, end: Idx) -> Self {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/parser/dotdotdot-expr.rs stdout ----
diff of stderr:

8    |
9 LL |     let _redemptive = 1..21;
- help: or `..=` for an inclusive range
+ help: use `..=` for an inclusive range
12    |
12    |
13 LL |     let _redemptive = 1..=21;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/dotdotdot-expr/dotdotdot-expr.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/dotdotdot-expr/dotdotdot-expr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/dotdotdot-expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/dotdotdot-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/dotdotdot-expr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/dotdotdot-expr/auxiliary"
stdout: none
--- stderr -------------------------------
error: unexpected token: `...`
   |
   |
LL |     let _redemptive = 1...21;
   |
help: use `..` for an exclusive range
   |
   |
LL |     let _redemptive = 1..21;
help: use `..=` for an inclusive range
   |
   |
LL |     let _redemptive = 1..=21;

error: aborting due to previous error
------------------------------------------

---
To only update this specific test, also pass `--test-args parser/issues/issue-90993.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-90993.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-90993" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-90993/auxiliary"
stdout: none
--- stderr -------------------------------
error: unexpected token: `...`
   |
LL |     ...=.
   |     ^^^
   |
---
   |
LL |     ..==.
   |     ~~~

error: unexpected `=` after inclusive range
   |
LL |     ...=.
   |     ^^^^ help: use `..=` instead
   |
   |
   = note: inclusive ranges end with a single equals sign (`..=`)

error: expected one of `-`, `;`, `}`, or path, found `.`
   |
LL |     ...=.
LL |     ...=.
   |         ^ expected one of `-`, `;`, `}`, or path
error: aborting due to 3 previous errors
------------------------------------------


---
55    |               ~~
- help: or `..=` for an inclusive range
+ help: use `..=` for an inclusive range
57    |
58 LL |     for _ in 0..=1 {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range_inclusive_dotdotdot/range_inclusive_dotdotdot.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range_inclusive_dotdotdot/range_inclusive_dotdotdot.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/range_inclusive_dotdotdot.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/range_inclusive_dotdotdot.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range_inclusive_dotdotdot" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range_inclusive_dotdotdot/auxiliary"
stdout: none
--- stderr -------------------------------
error: unexpected token: `...`
   |
   |
LL |     return ...1; //~ERROR unexpected token: `...`
   |
help: use `..` for an exclusive range
   |
   |
LL |     return ..1; //~ERROR unexpected token: `...`
help: use `..=` for an inclusive range
   |
   |
LL |     return ..=1; //~ERROR unexpected token: `...`

error: unexpected token: `...`
  --> /checkout/src/test/ui/parser/range_inclusive_dotdotdot.rs:12:13
   |
   |
LL |     let x = ...0;    //~ERROR unexpected token: `...`
   |
help: use `..` for an exclusive range
   |
   |
LL |     let x = ..0;    //~ERROR unexpected token: `...`
help: use `..=` for an inclusive range
   |
   |
LL |     let x = ..=0;    //~ERROR unexpected token: `...`

error: unexpected token: `...`
  --> /checkout/src/test/ui/parser/range_inclusive_dotdotdot.rs:16:14
   |
   |
LL |     let x = 5...5;   //~ERROR unexpected token: `...`
   |
help: use `..` for an exclusive range
   |
   |
LL |     let x = 5..5;   //~ERROR unexpected token: `...`
help: use `..=` for an inclusive range
   |
   |
LL |     let x = 5..=5;   //~ERROR unexpected token: `...`

error: unexpected token: `...`
  --> /checkout/src/test/ui/parser/range_inclusive_dotdotdot.rs:20:15
   |
   |
LL |     for _ in 0...1 {} //~ERROR unexpected token: `...`
   |
help: use `..` for an exclusive range
   |
   |
LL |     for _ in 0..1 {} //~ERROR unexpected token: `...`
help: use `..=` for an inclusive range
   |
   |
LL |     for _ in 0..=1 {} //~ERROR unexpected token: `...`

error: aborting due to 4 previous errors
------------------------------------------

