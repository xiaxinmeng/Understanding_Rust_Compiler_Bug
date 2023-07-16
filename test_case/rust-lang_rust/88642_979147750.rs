plain
.................................................................................................... 9600/12413
.................................................................................................... 9700/12413
.................................................................................................... 9800/12413
.................................................................................................... 9900/12413
..............................................................................F....F....i..ii.i..... 10000/12413
...F................................................................................................ 10100/12413
...i...........................F...................................................................i 10200/12413
.................................................................................................... 10400/12413
.................................................................................................... 10500/12413
.................................................................................................... 10600/12413
.................................................................................................... 10700/12413
---
failures:

---- [ui] ui/expr/if/attrs/let-chains-attr.rs stdout ----
normalized stderr:
warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(let_chains)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---
To only update this specific test, also pass `--test-args expr/if/attrs/let-chains-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/expr/if/attrs/let-chains-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/attrs/let-chains-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/attrs/let-chains-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(let_chains)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information


warning: 1 warning emitted


------------------------------------------


---- [ui] ui/rfc-2497-if-let-chains/irrefutable-lets.rs stdout ----
normalized stderr:
warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
  --> $DIR/irrefutable-lets.rs:3:12
   |
LL | #![feature(let_chains)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

---

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/irrefutable-lets/irrefutable-lets.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/irrefutable-lets.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/irrefutable-lets.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/irrefutable-lets" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/irrefutable-lets/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/irrefutable-lets.rs:3:12
   |
LL | #![feature(let_chains)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information


warning: 1 warning emitted


------------------------------------------


---- [ui] ui/rfc-2497-if-let-chains/u2-beautiful-day.rs stdout ----
normalized stderr:
warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
  --> $DIR/u2-beautiful-day.rs:3:12
   |
LL | #![feature(let_chains)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information


warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/u2-beautiful-day/u2-beautiful-day.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/u2-beautiful-day.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/u2-beautiful-day.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/u2-beautiful-day" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/u2-beautiful-day/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/u2-beautiful-day.rs:3:12
   |
LL | #![feature(let_chains)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information


warning: 1 warning emitted


------------------------------------------


---- [ui] ui/rfc-2497-if-let-chains/then-else-blocks.rs stdout ----
normalized stderr:
warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
  --> $DIR/then-else-blocks.rs:3:12
   |
LL | #![feature(let_chains)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information


warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/then-else-blocks/then-else-blocks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/then-else-blocks.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/then-else-blocks.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/then-else-blocks/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/then-else-blocks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/then-else-blocks.rs:3:12
   |
LL | #![feature(let_chains)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

---

---- [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs stdout ----
diff of stderr:

504    = note: only supported directly in conditions of `if`- and `while`-expressions
505    = note: as well as when nested within `&&` and parentheses in those conditions
506 
+ warning: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
+    |
+    |
+ LL | #![feature(let_chains)] // Avoid inflating `.stderr` with overzealous gates in this test.
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
+ 
+ 
507 error[E0308]: mismatched types
508   --> $DIR/disallowed-positions.rs:29:8
509    |

945    |
946    = help: the trait `Try` is not implemented for `{integer}`
- error: aborting due to 103 previous errors
+ error: aborting due to 103 previous errors; 1 warning emitted
949 
950 Some errors have detailed explanations: E0277, E0308, E0600, E0614.
---
To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expressions must be enclosed in braces to be used as const generic arguments
   |
   |
LL |         true && let 1 = 1
   |
   |
help: enclose the `const` expression in braces
   |
LL |         { true && let 1 = 1 }
   |         +                   +

error: `let` expressions are not supported here
   |
   |
LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: _, end: _ } = true..true && false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: _, end: _ } = true..true || false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: F, end } = F..|| true {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: true, end } = t..&&false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let Range { start: _, end: _ } = true..true && false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let Range { start: _, end: _ } = true..true || false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let Range { start: F, end } = F..|| true {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let Range { start: true, end } = t..&&false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     ..(let 0 = 0); //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (let 0 = 0)..; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (let Range { start: _, end: _ } = true..true || false);
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
LL |     (let true = let true = true);
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     &let 0 = 0
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
   |
   |
LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parentheses in those conditions

error: `let` expressions are not supported here
---

failures:
    [ui] ui/expr/if/attrs/let-chains-attr.rs
    [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs
    [ui] ui/rfc-2497-if-let-chains/irrefutable-lets.rs
    [ui] ui/rfc-2497-if-let-chains/then-else-blocks.rs
    [ui] ui/rfc-2497-if-let-chains/u2-beautiful-day.rs
test result: FAILED. 12297 passed; 5 failed; 111 ignored; 0 measured; 0 filtered out; finished in 140.05s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:15
