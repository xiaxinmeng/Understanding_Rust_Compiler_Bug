plain

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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
help: consider adding 'move' keyword before the nested closure
   |
LL |     move |()| s.chars().map(move |c| format!("{}{}", c, s))

error: aborting due to 2 previous errors
------------------------------------------

