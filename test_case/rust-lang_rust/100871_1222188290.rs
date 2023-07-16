plain
........................................................................................ 6952/13411
.................................i....i..........................................i...... 7040/13411
...........i.............i.........................................................i.... 7128/13411
........................................................................................ 7216/13411
..............i...............................................F..........F.............. 7304/13411
..F................F..........F.....F.....................F............................. 7392/13411
...........ii.......................................ii.................................. 7568/13411
..............................i......................................................... 7656/13411
........................................................................................ 7744/13411
......................................ii................................................ 7832/13411
---

---- [ui] src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs stdout ----
diff of stderr:

7 LL |         .flat_map(move |()| s.chars().map(|c| format!("{}{}", c, s)))
8    |                           - -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
9    |                           | |
-    |                           | returns a reference to a captured variable which escapes the closure body
11    |                           | variable captured here
+    |                           | returns a reference to a captured variable which escapes the closure body
12    |                           inferred to be a `FnMut` closure
13    |
14    = note: `FnMut` closures only have access to their captured variables while they are executing...

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure/issue-95079-missing-move-in-nested-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-95079-missing-move-in-nested-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error: captured variable cannot escape `FnMut` closure body
   |
   |
LL | fn foo1(s: &str) -> impl Iterator<Item = String> + '_ {
   |         - variable defined here
LL |     None.into_iter()
LL |         .flat_map(move |()| s.chars().map(|c| format!("{}{}", c, s)))
   |                           - -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                           | variable captured here
   |                           | variable captured here
   |                           | returns a reference to a captured variable which escapes the closure body
   |                           inferred to be a `FnMut` closure
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
help: consider adding 'move' keyword before the nested closure
   |
LL |         .flat_map(move |()| s.chars().map(move |c| format!("{}{}", c, s)))

error: lifetime may not live long enough
  --> /checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs:9:15
   |
   |
LL |     move |()| s.chars().map(|c| format!("{}{}", c, s))
   |     --------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |     |       |
   |     |       return type of closure `Map<Chars<'_>, [closure@/checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs:9:29: 9:32]>` contains a lifetime `'2`
   |     lifetime `'1` represents this closure's body
   |
   = note: closure implements `Fn`, so references to captured variables can't escape the closure
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
help: consider adding 'move' keyword before the nested closure
   |
LL |     move |()| s.chars().map(move |c| format!("{}{}", c, s))

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/closure-expected-type/expect-fn-supply-fn.rs stdout ----
diff of stderr:

7 LL |     with_closure_expecting_fn_with_free_region(|x: fn(&'x u32), y| {});
9    |                                                 |
9    |                                                 |
-    |                                                 has type `fn(&'1 u32)`
11    |                                                 requires that `'1` must outlive `'x`
+    |                                                 has type `fn(&'1 u32)`
13 error: lifetime may not live long enough
14   --> $DIR/expect-fn-supply-fn.rs:16:49



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn/expect-fn-supply-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closure-expected-type/expect-fn-supply-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn expect_free_supply_free_from_fn<'x>(x: &'x u32) {
   |                                    -- lifetime `'x` defined here
...
LL |     with_closure_expecting_fn_with_free_region(|x: fn(&'x u32), y| {});
   |                                                 |
   |                                                 |
   |                                                 requires that `'1` must outlive `'x`
   |                                                 has type `fn(&'1 u32)`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:16:49
   |
   |
LL | fn expect_free_supply_free_from_fn<'x>(x: &'x u32) {
   |                                    -- lifetime `'x` defined here
...
LL |     with_closure_expecting_fn_with_free_region(|x: fn(&'x u32), y| {});
   |                                                 ^ requires that `'x` must outlive `'static`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:32:49
   |
   |
LL |     with_closure_expecting_fn_with_free_region(|x: fn(&u32), y| {});
   |                                                 ^ one type is more general than the other
   |
   = note: expected fn pointer `for<'r> fn(&'r u32)`
              found fn pointer `fn(&u32)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:39:50
   |
   |
LL |     with_closure_expecting_fn_with_bound_region(|x: fn(&'x u32), y| {});
   |                                                  ^ one type is more general than the other
   |
   = note: expected fn pointer `fn(&'x u32)`
              found fn pointer `for<'r> fn(&'r u32)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:48:50
   |
   |
LL |     with_closure_expecting_fn_with_bound_region(|x: Foo<'_>, y| {
   |                                                  ^ one type is more general than the other
   = note: expected fn pointer `fn(&u32)`
   = note: expected fn pointer `fn(&u32)`
              found fn pointer `for<'r> fn(&'r u32)`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
-    |             |
-    |             help: remove this `mut`
+    |             ^^^^^ help: remove this `mut`
20    |
21    = note: this overrides the previous `expect` lint level and warns about the `unused_mut` lint here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels/expect_nested_lint_levels.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels/expect_nested_lint_levels.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels/auxiliary"
stdout: none
--- stderr -------------------------------
error: unused variable: `this_is_my_function`
   |
   |
LL |     let this_is_my_function = 3;
   |         ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_this_is_my_function`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:45:10
   |
   |
LL | #[forbid(unused_variables)]

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:36:13
   |
   |
LL |         let mut v = 0;
   |             ^^^^^ help: remove this `mut`
   |
   = note: this overrides the previous `expect` lint level and warns about the `unused_mut` lint here
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:31:9
   |
LL |         unused_mut,
   |         ^^^^^^^^^^
   |         ^^^^^^^^^^

warning: this lint expectation is unfulfilled
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:7:5
   |
LL |     unused_mut,
   |     ^^^^^^^^^^
   |
   = note: `#[warn(unfulfilled_lint_expectations)]` on by default
   = note: this `expect` is overridden by a `allow` attribute before the `unused_mut` lint is triggered
warning: this lint expectation is unfulfilled
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:24:5
   |
LL |     unused_mut,
LL |     unused_mut,
   |     ^^^^^^^^^^
   |
   = note: this `expect` is overridden by a `warn` attribute before the `unused_mut` lint is triggered
warning: this lint expectation is unfulfilled
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:43:10
   |
   |
LL | #[expect(unused_variables)]

error: aborting due to previous error; 4 warnings emitted
------------------------------------------



---- [ui] src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs stdout ----
diff of stderr:

30   --> $DIR/force_warn_expected_lints_fulfilled.rs:32:9
31    |
32 LL |     let mut what_does_the_fox_say = "*ding* *deng* *dung*";
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove this `mut`
36    |
36    |
37    = note: requested on the command line with `--force-warn unused-mut`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled/force_warn_expected_lints_fulfilled.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled/force_warn_expected_lints_fulfilled.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warn" "while_true" "--force-warn" "unused_variables" "--force-warn" "unused_mut" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
LL |     while true {
   |     ^^^^^^^^^^ help: use `loop`
   |
   |
   = note: requested on the command line with `--force-warn while-true`
warning: unused variable: `x`
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs:20:9
   |
LL |     let x = 2;
LL |     let x = 2;
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
   = note: requested on the command line with `--force-warn unused-variables`

warning: unused variable: `fox_name`
   |
   |
LL |     let fox_name = "Sir Nibbles";
   |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_fox_name`
warning: unused variable: `this_should_fulfill_the_expectation`
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs:43:9
   |
   |
LL |     let this_should_fulfill_the_expectation = "The `#[allow]` has no power here";
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_this_should_fulfill_the_expectation`
warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs:32:9
   |
   |
LL |     let mut what_does_the_fox_say = "*ding* *deng* *dung*";
   |
   |
   = note: requested on the command line with `--force-warn unused-mut`
warning: 5 warnings emitted
------------------------------------------



---- [ui] src/test/ui/lint/suggestions.rs stdout ----
diff of stderr:

27   --> $DIR/suggestions.rs:48:13
28    |
29 LL |         let mut registry_no = (format!("NX-{}", 74205));
-    |             |
-    |             help: remove this `mut`
+    |             ^^^^^^^^^^^^^^^ help: remove this `mut`
33    |
---
+    |  _____________^
+ LL | |             b = 1;
+    | |_____________^ help: remove this `mut`
51 
52 error: const items should never be `#[no_mangle]`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/suggestions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/suggestions.stderr
diff of fixed:

45     loop {
46     //~^ WARN denote infinite loops
47     //~| HELP use `loop`
-         let registry_no = format!("NX-{}", 74205);
+         let  = format!("NX-{}", 74205);
49         //~^ WARN does not need to be mutable
50         //~| HELP remove this `mut`
51         //~| WARN unnecessary parentheses

52         //~| HELP remove these parentheses
53         // the line after `mut` has a `\t` at the beginning, this is on purpose
-         let b = 1;
+         let  = 1;
55         //~^^ WARN does not need to be mutable
56         //~| HELP remove this `mut`
57         let d = Equinox { warp_factor: 9.975 };

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/suggestions.fixed
To only update this specific test, also pass `--test-args lint/suggestions.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
LL |     while true {
   |     ^^^^^^^^^^ help: use `loop`
   |
   |
   = note: `#[warn(while_true)]` on by default

warning: unnecessary parentheses around assigned value
  --> /checkout/src/test/ui/lint/suggestions.rs:48:31
   |
LL |         let mut registry_no = (format!("NX-{}", 74205));
   |                               ^                       ^
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/suggestions.rs:4:21
   |
   |
LL | #![warn(unused_mut, unused_parens)] // UI tests pass `-A unused`—see Issue #43896
help: remove these parentheses
   |
   |
LL -         let mut registry_no = (format!("NX-{}", 74205));
LL +         let mut registry_no = format!("NX-{}", 74205);

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/suggestions.rs:48:13
   |
   |
LL |         let mut registry_no = (format!("NX-{}", 74205));
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/suggestions.rs:4:9
   |
   |
LL | #![warn(unused_mut, unused_parens)] // UI tests pass `-A unused`—see Issue #43896

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/suggestions.rs:54:13
   |
   |
LL |           let mut
   |  _____________^
LL | |             b = 1;
   | |_____________^ help: remove this `mut`

error: const items should never be `#[no_mangle]`
   |
   |
LL | #[no_mangle] const DISCOVERY: usize = 1;
   |              |
   |              |
   |              help: try a static value: `pub static`
   |
   = note: `#[deny(no_mangle_const_items)]` on by default
warning: functions generic over types or consts must be mangled
  --> /checkout/src/test/ui/lint/suggestions.rs:12:1
   |
   |
LL | #[no_mangle]
   | ------------ help: remove this attribute
LL | //~^ HELP remove this attribute
LL | pub fn defiant<T>(_t: T) {}
   |
   = note: `#[warn(no_mangle_generic_items)]` on by default


warning: the `warp_factor:` in this pattern is redundant
   |
   |
LL |             Equinox { warp_factor: warp_factor } => {}
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^ help: use shorthand field pattern: `warp_factor`
   = note: `#[warn(non_shorthand_field_patterns)]` on by default


error: const items should never be `#[no_mangle]`
   |
   |
LL |     #[no_mangle] pub const DAUNTLESS: bool = true;
   |                  |
   |                  |
   |                  help: try a static value: `pub static`
warning: functions generic over types or consts must be mangled
  --> /checkout/src/test/ui/lint/suggestions.rs:26:18
   |
   |
LL |     #[no_mangle] pub fn val_jean<T>() {}
   |     |
   |     help: remove this attribute


error: const items should never be `#[no_mangle]`
   |
   |
LL |     #[no_mangle] pub(crate) const VETAR: bool = true;
   |                  |
   |                  |
   |                  help: try a static value: `pub static`
warning: functions generic over types or consts must be mangled
  --> /checkout/src/test/ui/lint/suggestions.rs:35:18
   |
   |
LL |     #[no_mangle] pub(crate) fn crossfield<T>() {}
   |     |
   |     help: remove this attribute

error: aborting due to 3 previous errors; 8 warnings emitted
---
106 

108   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:37:10
109    |
110 LL |     let (mut var, unused_var) = (1, 2);
-    |          |
-    |          help: remove this `mut`
+    |          ^^^^^^^ help: remove this `mut`
114 
---
To only update this specific test, also pass `--test-args lint/unused/issue-47390-unused-variable-in-struct-pattern.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `i_think_continually`
   |
   |
LL |     let i_think_continually = 2; //~ WARNING unused variable: `i_think_continually`
   |         ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_i_think_continually`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:5:9
   |
   |
LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
   = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`

warning: unused variable: `mut_unused_var`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:13
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:13
   |
LL |     let mut mut_unused_var = 1;
   |             ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_mut_unused_var`

warning: unused variable: `var`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:14
   |
LL |     let (mut var, unused_var) = (1, 2);
   |              ^^^ help: if this is intentional, prefix it with an underscore: `_var`
warning: unused variable: `unused_var`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:19
   |
   |
LL |     let (mut var, unused_var) = (1, 2);
   |                   ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused_var`

warning: unused variable: `corridors_of_light`
   |
   |
LL |     if let SoulHistory { corridors_of_light, //~ WARNING unused variable: `corridors_of_light`
   |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`

warning: variable `hours_are_suns` is assigned to, but never used
   |
   |
LL |                          mut hours_are_suns, //~ WARNING `hours_are_suns` is assigned to, but
   |
   |
   = note: consider using `_hours_are_suns` instead

warning: value assigned to `hours_are_suns` is never read
   |
   |
LL |         hours_are_suns = false; //~ WARNING unused_assignments
   |
   = note: `#[warn(unused_assignments)]` implied by `#[warn(unused)]`
   = note: `#[warn(unused_assignments)]` implied by `#[warn(unused)]`
   = help: maybe it is overwritten before being read?
warning: unused variable: `fire`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:52:32
   |
   |
LL |     let LovelyAmbition { lips, fire } = the_spirit; //~ WARNING unused variable: `fire`
   |                                ^^^^ help: try ignoring the field: `fire: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:61:23
   |
   |
LL |         Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                       ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:66:24
   |
   |
LL |         &Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:71:27
   |
   |
LL |         box Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                           ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:76:24
   |
   |
LL |         (Large::Suit { case },) => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:81:24
   |
   |
LL |         [Large::Suit { case }] => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:86:29
   |
   |
LL |         Tuple(Large::Suit { case }, ()) => {} //~ WARNING unused variable: `case`
   |                             ^^^^ help: try ignoring the field: `case: _`
warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:9
   |
LL |     let mut mut_unused_var = 1;
LL |     let mut mut_unused_var = 1;
   |         ^^^^^^^^^^^^^^^^^^ help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` implied by `#[warn(unused)]`

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:10
   |
LL |     let (mut var, unused_var) = (1, 2);

warning: 16 warnings emitted
------------------------------------------

---
10   --> $DIR/lint-unused-mut-self.rs:6:9

16   --> $DIR/lint-unused-mut-self.rs:11:12
17    |
18 LL |     fn bar(mut self: Box<Foo>) {}
-    |            |
-    |            help: remove this `mut`
+    |            ^^^^^^^^ help: remove this `mut`
22 
---

7 
8 struct Foo;
9 impl Foo {
-     fn foo(self) {} //~ ERROR: variable does not need to be mutable
-     fn bar(self: Box<Foo>) {} //~ ERROR: variable does not need to be mutable
+     fn foo() {} //~ ERROR: variable does not need to be mutable
+     fn bar(: Box<Foo>) {} //~ ERROR: variable does not need to be mutable
13 
14 fn main() {}



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-self/lint-unused-mut-self.fixed
To only update this specific test, also pass `--test-args lint/unused/lint-unused-mut-self.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/lint-unused-mut-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-self" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-self/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
   |
LL |     fn foo(mut self) {} //~ ERROR: variable does not need to be mutable
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-self.rs:6:9
   |
   |
LL | #![deny(unused_mut)]
   |         ^^^^^^^^^^

error: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-self.rs:11:12
   |
LL |     fn bar(mut self: Box<Foo>) {} //~ ERROR: variable does not need to be mutable

error: aborting due to 2 previous errors
------------------------------------------

---
71 warning: variable does not need to be mutable
72   --> $DIR/lint-unused-mut-variables.rs:107:14

73    |
74 LL |     let x = |mut y: isize| 10;
-    |              |
-    |              help: remove this `mut`
+    |              ^^^^^ help: remove this `mut`
78 
---
103 warning: variable does not need to be mutable
104   --> $DIR/lint-unused-mut-variables.rs:75:9

105    |
106 LL |     let mut a = vec![3];
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^ help: remove this `mut`
110 
110 
111 warning: variable does not need to be mutable
112   --> $DIR/lint-unused-mut-variables.rs:77:10

113    |
114 LL |     let (mut a, b) = (1, 2);
-    |          |
-    |          help: remove this `mut`
+    |          ^^^^^ help: remove this `mut`
118 
---
143 warning: variable does not need to be mutable
144   --> $DIR/lint-unused-mut-variables.rs:99:10

145    |
146 LL |         (mut x, 1) |
-    |          |
-    |          help: remove this `mut`
+    |          ^^^^^ help: remove this `mut`
150 
---
159 warning: variable does not need to be mutable
160   --> $DIR/lint-unused-mut-variables.rs:117:9

161    |
162 LL |     let mut b = (&mut a,);
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^ help: remove this `mut`
166 
---
175 warning: variable does not need to be mutable
176   --> $DIR/lint-unused-mut-variables.rs:132:9

177    |
178 LL |     let mut v : &mut Vec<()> = &mut vec![];
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^ help: remove this `mut`
182 
---
191 warning: variable does not need to be mutable
192   --> $DIR/lint-unused-mut-variables.rs:109:13

193    |
194 LL |     fn what(mut foo: isize) {}
-    |             |
-    |             help: remove this `mut`
+    |             ^^^^^^^ help: remove this `mut`
198 
198 
199 warning: variable does not need to be mutable
200   --> $DIR/lint-unused-mut-variables.rs:127:20

201    |
202 LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
-    |                    |
-    |                    help: remove this `mut`
+    |                    ^^^^^^^ help: remove this `mut`
206 
206 
207 error: variable does not need to be mutable
208   --> $DIR/lint-unused-mut-variables.rs:205:9

209    |
210 LL |     let mut b = vec![2];
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^ help: remove this `mut`
214    |
---
To only update this specific test, also pass `--test-args lint/unused/lint-unused-mut-variables.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-variables" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-variables/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:9:5
   |
LL |     mut a: i32,
   |     ^^^^^ help: remove this `mut`
---

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:107:14
   |
LL |     let x = |mut y: isize| 10; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:69:9
   |
   |
LL |     let mut a = 3; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:71:9
   |
   |
LL |     let mut a = 2; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:73:9
   |
   |
LL |     let mut b = 3; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:75:9
   |
   |
LL |     let mut a = vec![3]; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:77:10
   |
   |
LL |     let (mut a, b) = (1, 2); //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:79:9
   |
   |
LL |     let mut a; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:83:9
   |
   |
LL |     let mut b; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:92:9
   |
   |
LL |         mut x => {} //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:99:10
   |
   |
LL |         (mut x, 1) | //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:112:9
   |
   |
LL |     let mut a = &mut 5; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:117:9
   |
   |
LL |     let mut b = (&mut a,); //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:120:9
   |
   |
LL |     let mut x = &mut 1; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:132:9
   |
   |
LL |     let mut v : &mut Vec<()> = &mut vec![]; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:187:9
   |
   |
LL |     let mut raw_address_of_const = 1; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:109:13
   |
   |
LL |     fn what(mut foo: isize) {} //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:127:20
   |
   |
LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {

error: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:205:9
   |
   |
LL |     let mut b = vec![2]; //~ ERROR: variable does not need to be mutable
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:201:8
   |
---
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-mut-warning-captured-var/unused-mut-warning-captured-var.stderr
diff of fixed:

3 #![forbid(unused_mut)]
5 fn main() {
-     let x = 1;
+     let  = 1;
+     let  = 1;
7     //~^ ERROR: variable does not need to be mutable
8     (move|| { println!("{}", x); })();


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-mut-warning-captured-var/unused-mut-warning-captured-var.fixed
To only update this specific test, also pass `--test-args lint/unused/unused-mut-warning-captured-var.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-mut-warning-captured-var.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-mut-warning-captured-var" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-mut-warning-captured-var/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
LL |     let mut x = 1;
   |         ^^^^^ help: remove this `mut`
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-mut-warning-captured-var.rs:3:11
   |
LL | #![forbid(unused_mut)]

error: aborting due to previous error
------------------------------------------

---
diff of fixed:

6 #![deny(unused_mut)]
7 
8 pub fn mutable_upvar() {
-     let x = &mut 0;
+     let  = &mut 0;
10     //~^ ERROR
11     let _ = move || {
12         *x = 1;

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref/capture-mut-ref.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/capture-mut-ref.rs`
error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/capture-mut-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
LL |     let mut x = &mut 0;
   |         ^^^^^ help: remove this `mut`
   |
---

3 #![deny(unused_mut)]
4 
5 fn main() {
-     let x; //~ ERROR: variable does not need to be mutable
+     let ; //~ ERROR: variable does not need to be mutable
7     x = String::new();
8     dbg!(x);


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424/issue-61424.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-61424.rs`
error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-61424.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
   |
LL |     let mut x; //~ ERROR: variable does not need to be mutable
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/nll/issue-61424.rs:3:9
   |
---
diff of stderr:

2   --> $DIR/unused-mut-issue-50343.rs:7:33
3    |
4 LL |     vec![(42, 22)].iter().map(|(mut x, _y)| ()).count();
-    |                                 |
-    |                                 help: remove this `mut`
+    |                                 ^^^^^ help: remove this `mut`
8    |
---

4 #![allow(unused_variables)] // for rustfix
5 
6 fn main() {
-     vec![(42, 22)].iter().map(|(x, _y)| ()).count();
+     vec![(42, 22)].iter().map(|(, _y)| ()).count();
8     //~^ ERROR: variable does not need to be mutable
10 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343/unused-mut-issue-50343.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/unused-mut-issue-50343.rs`
error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/unused-mut-issue-50343.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
   |
LL |     vec![(42, 22)].iter().map(|(mut x, _y)| ()).count();
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/nll/unused-mut-issue-50343.rs:3:9
   |
---
diff of stderr:

2   --> $DIR/issue-73592-borrow_mut-through-deref.rs:42:17
3    |
4 LL | fn test_mut_pin(mut s: Pin<&S>) {
-    |                 |
-    |                 help: remove this `mut`
+    |                 ^^^^^ help: remove this `mut`
8    |
8    |
9 note: the lint level is defined here
10   --> $DIR/issue-73592-borrow_mut-through-deref.rs:20:9

16   --> $DIR/issue-73592-borrow_mut-through-deref.rs:47:21
17    |
18 LL | fn test_mut_pin_mut(mut s: Pin<&mut S>) {
-    |                     |
-    |                     help: remove this `mut`
+    |                     ^^^^^ help: remove this `mut`
22 
---
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-73592-borrow_mut-through-deref/issue-73592-borrow_mut-through-deref.stderr
diff of fixed:

39     let _ = &mut *s[0].borrow_mut();
41 
41 
- fn test_mut_pin(s: Pin<&S>) {
+ fn test_mut_pin(: Pin<&S>) {
43     //~^ WARN variable does not need to be mutable
44     let _ = &mut *s.0.borrow_mut();

46 
46 
- fn test_mut_pin_mut(s: Pin<&mut S>) {
+ fn test_mut_pin_mut(: Pin<&mut S>) {
48     //~^ WARN variable does not need to be mutable
49     let _ = &mut *s.0.borrow_mut();


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-73592-borrow_mut-through-deref/issue-73592-borrow_mut-through-deref.fixed
To only update this specific test, also pass `--test-args typeck/issue-73592-borrow_mut-through-deref.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-73592-borrow_mut-through-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-73592-borrow_mut-through-deref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-73592-borrow_mut-through-deref/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/typeck/issue-73592-borrow_mut-through-deref.rs:42:17
   |
   |
LL | fn test_mut_pin(mut s: Pin<&S>) {
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/typeck/issue-73592-borrow_mut-through-deref.rs:20:9
   |
   |
LL | #![warn(unused_mut)]
   |         ^^^^^^^^^^

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/typeck/issue-73592-borrow_mut-through-deref.rs:47:21
   |
LL | fn test_mut_pin_mut(mut s: Pin<&mut S>) {

warning: 2 warnings emitted
------------------------------------------

