plain
........................................................................................ 4576/13119
........................................................................................ 4664/13119
........................................................................................ 4752/13119
........................................................................................ 4840/13119
.................................F...........F.......................F..F........F...FF. 4928/13119
.........................F.............................................................. 5016/13119
........................................................................................ 5192/13119
........................................................................................ 5280/13119
........................................................................................ 5368/13119
........................................................................................ 5456/13119
---
8   --> $DIR/feature-gate-extern_absolute_paths.rs:4:19
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

9    |
10 LL |     let _: u8 = ::core::default::Default();
11    |                   ^^^^ maybe a missing crate `core`?
+    = help: consider adding `extern crate core` to use the `core` crate
12 
13 error: aborting due to 2 previous errors
14 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-extern_absolute_paths.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-extern_absolute_paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-extern_absolute_paths" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-extern_absolute_paths/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/feature-gates/feature-gate-extern_absolute_paths.rs:1:5
   |
   |
LL | use core::default; //~ ERROR unresolved import `core`
   |     ^^^^ maybe a missing crate `core`?
   = help: consider adding `extern crate core` to use the `core` crate

error[E0433]: failed to resolve: maybe a missing crate `core`?
  --> /checkout/src/test/ui/feature-gates/feature-gate-extern_absolute_paths.rs:4:19
  --> /checkout/src/test/ui/feature-gates/feature-gate-extern_absolute_paths.rs:4:19
   |
LL |     let _: u8 = ::core::default::Default(); //~ ERROR failed to resolve
   |                   ^^^^ maybe a missing crate `core`?
   = help: consider adding `extern crate core` to use the `core` crate

error: aborting due to 2 previous errors

---
3    |
4 LL | use main::bar;
5    |     ^^^^ maybe a missing crate `main`?
+    |
+    = help: consider adding `extern crate main` to use the `main` crate
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import3/import3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/import3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/import3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `main`
   |
LL | use main::bar;
   |     ^^^^ maybe a missing crate `main`?
   |
   |
   = help: consider adding `extern crate main` to use the `main` crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/imports/issue-1697.rs stdout ----
diff of stderr:

3    |
4 LL | use unresolved::*;
5    |     ^^^^^^^^^^ maybe a missing crate `unresolved`?
+    |
+    = help: consider adding `extern crate unresolved` to use the `unresolved` crate
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-1697/issue-1697.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-1697.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-1697.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-1697" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-1697/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `unresolved`
   |
   |
LL | use unresolved::*; //~ ERROR unresolved import `unresolved` [E0432]
   |     ^^^^^^^^^^ maybe a missing crate `unresolved`?
   |
   = help: consider adding `extern crate unresolved` to use the `unresolved` crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/imports/issue-33464.rs stdout ----
diff of stderr:

3    |
4 LL | use abc::one_el;
5    |     ^^^ maybe a missing crate `abc`?
+    |
+    = help: consider adding `extern crate abc` to use the `abc` crate
7 error[E0432]: unresolved import `abc`
8   --> $DIR/issue-33464.rs:5:5

9    |
9    |
10 LL | use abc::{a, bbb, cccccc};
11    |     ^^^ maybe a missing crate `abc`?
+    |
+    = help: consider adding `extern crate abc` to use the `abc` crate
12 
13 error[E0432]: unresolved import `a_very_long_name`

15    |
15    |
16 LL | use a_very_long_name::{el, el2};
17    |     ^^^^^^^^^^^^^^^^ maybe a missing crate `a_very_long_name`?
+    |
+    = help: consider adding `extern crate a_very_long_name` to use the `a_very_long_name` crate
19 error: aborting due to 3 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-33464/issue-33464.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-33464.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-33464.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-33464" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-33464/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `abc`
   |
   |
LL | use abc::one_el;
   |     ^^^ maybe a missing crate `abc`?
   |
   = help: consider adding `extern crate abc` to use the `abc` crate
error[E0432]: unresolved import `abc`
  --> /checkout/src/test/ui/imports/issue-33464.rs:5:5
   |
   |
LL | use abc::{a, bbb, cccccc};
   |     ^^^ maybe a missing crate `abc`?
   |
   = help: consider adding `extern crate abc` to use the `abc` crate

error[E0432]: unresolved import `a_very_long_name`
   |
   |
LL | use a_very_long_name::{el, el2};
   |     ^^^^^^^^^^^^^^^^ maybe a missing crate `a_very_long_name`?
   |
   = help: consider adding `extern crate a_very_long_name` to use the `a_very_long_name` crate
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/imports/issue-37887.rs stdout ----
diff of stderr:

3    |
4 LL |     use libc::*;
5    |         ^^^^ maybe a missing crate `libc`?
+    |
+    = help: consider adding `extern crate libc` to use the `libc` crate
6 
7 error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-37887/issue-37887.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-37887/issue-37887.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-37887.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-37887.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-37887" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-37887/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `libc`
   |
   |
LL |     use libc::*; //~ ERROR unresolved import
   |         ^^^^ maybe a missing crate `libc`?
   |
   = help: consider adding `extern crate libc` to use the `libc` crate

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
   |
LL |     extern crate libc; //~ ERROR use of unstable
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

---
---- [ui] src/test/ui/imports/issue-36881.rs stdout ----
diff of stderr:

3    |
4 LL |     use issue_36881_aux::Foo;
5    |         ^^^^^^^^^^^^^^^ maybe a missing crate `issue_36881_aux`?
+    |
+    = help: consider adding `extern crate issue_36881_aux` to use the `issue_36881_aux` crate
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-36881/issue-36881.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-36881.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-36881.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-36881" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-36881/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `issue_36881_aux`
   |
   |
LL |     use issue_36881_aux::Foo; //~ ERROR unresolved import
   |         ^^^^^^^^^^^^^^^ maybe a missing crate `issue_36881_aux`?
   |
   = help: consider adding `extern crate issue_36881_aux` to use the `issue_36881_aux` crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/imports/issue-53269.rs stdout ----
diff of stderr:

3    |
4 LL |     use nonexistent_module::mac;
5    |         ^^^^^^^^^^^^^^^^^^ maybe a missing crate `nonexistent_module`?
+    |
+    = help: consider adding `extern crate nonexistent_module` to use the `nonexistent_module` crate
6 
7 error[E0659]: `mac` is ambiguous


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269/issue-53269.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269/issue-53269.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-53269.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-53269.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `nonexistent_module`
   |
   |
LL |     use nonexistent_module::mac; //~ ERROR unresolved import `nonexistent_module`
   |         ^^^^^^^^^^^^^^^^^^ maybe a missing crate `nonexistent_module`?
   |
   = help: consider adding `extern crate nonexistent_module` to use the `nonexistent_module` crate

error[E0659]: `mac` is ambiguous
   |
   |
LL |     mac!(); //~ ERROR `mac` is ambiguous
   |     ^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a `macro_rules` name and a non-`macro_rules` name from another module
note: `mac` could refer to the macro defined here
   |
   |
LL | macro_rules! mac { () => () }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `mac` could also refer to the unresolved item imported here
   |
   |
LL |     use nonexistent_module::mac; //~ ERROR unresolved import `nonexistent_module`
   |         ^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `self::mac` to refer to this unresolved item unambiguously
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0432, E0659.
For more information about an error, try `rustc --explain E0432`.
For more information about an error, try `rustc --explain E0432`.
------------------------------------------


---- [ui] src/test/ui/imports/issue-55457.rs stdout ----
diff of stderr:

12    |
13 LL | use non_existent::non_existent;
14    |     ^^^^^^^^^^^^ maybe a missing crate `non_existent`?
+    |
+    = help: consider adding `extern crate non_existent` to use the `non_existent` crate
16 error: cannot determine resolution for the derive macro `NonExistent`
17   --> $DIR/issue-55457.rs:5:10



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-55457/issue-55457.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-55457.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-55457.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-55457" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-55457/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `NonExistent`
   |
   |
LL | use NonExistent; //~ ERROR unresolved import `NonExistent`
   |     |
   |     no `NonExistent` in the root
   |     help: a similar name exists in the module: `non_existent`


error[E0432]: unresolved import `non_existent`
  --> /checkout/src/test/ui/imports/issue-55457.rs:2:5
   |
LL | use non_existent::non_existent; //~ ERROR unresolved import `non_existent`
   |     ^^^^^^^^^^^^ maybe a missing crate `non_existent`?
   |
   = help: consider adding `extern crate non_existent` to use the `non_existent` crate
error: cannot determine resolution for the derive macro `NonExistent`
  --> /checkout/src/test/ui/imports/issue-55457.rs:5:10
   |
   |
LL | #[derive(NonExistent)] //~ ERROR cannot determine resolution for the derive macro `NonExistent`
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the attribute macro `non_existent`
  --> /checkout/src/test/ui/imports/issue-55457.rs:4:3
   |
   |
LL | #[non_existent] //~ ERROR cannot determine resolution for the attribute macro `non_existent`
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the derive macro `NonExistent`
  --> /checkout/src/test/ui/imports/issue-55457.rs:5:10
   |
   |
LL | #[derive(NonExistent)] //~ ERROR cannot determine resolution for the derive macro `NonExistent`
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the derive macro `NonExistent`
  --> /checkout/src/test/ui/imports/issue-55457.rs:5:10
   |
   |
LL | #[derive(NonExistent)] //~ ERROR cannot determine resolution for the derive macro `NonExistent`
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/imports/tool-mod-child.rs stdout ----
diff of stderr:

3    |
4 LL | use clippy::a::b;
5    |     ^^^^^^ maybe a missing crate `clippy`?
+    |
+    = help: consider adding `extern crate clippy` to use the `clippy` crate
7 error[E0432]: unresolved import `clippy`
8   --> $DIR/tool-mod-child.rs:1:5

9    |
9    |
10 LL | use clippy::a;
11    |     ^^^^^^ maybe a missing crate `clippy`?
+    |
+    = help: consider adding `extern crate clippy` to use the `clippy` crate
12 
13 error[E0433]: failed to resolve: maybe a missing crate `rustdoc`?

15    |
15    |
16 LL | use rustdoc::a::b;
17    |     ^^^^^^^ maybe a missing crate `rustdoc`?
+    |
+    = help: consider adding `extern crate rustdoc` to use the `rustdoc` crate
19 error[E0432]: unresolved import `rustdoc`
20   --> $DIR/tool-mod-child.rs:4:5

21    |
21    |
22 LL | use rustdoc::a;
23    |     ^^^^^^^ maybe a missing crate `rustdoc`?
+    |
+    = help: consider adding `extern crate rustdoc` to use the `rustdoc` crate
25 error: aborting due to 4 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/tool-mod-child/tool-mod-child.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/tool-mod-child.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/tool-mod-child.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/tool-mod-child" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/tool-mod-child/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: maybe a missing crate `clippy`?
   |
   |
LL | use clippy::a::b; //~ ERROR failed to resolve: maybe a missing crate `clippy`?
   |     ^^^^^^ maybe a missing crate `clippy`?
   |
   = help: consider adding `extern crate clippy` to use the `clippy` crate
error[E0432]: unresolved import `clippy`
  --> /checkout/src/test/ui/imports/tool-mod-child.rs:1:5
   |
   |
LL | use clippy::a; //~ ERROR unresolved import `clippy`
   |     ^^^^^^ maybe a missing crate `clippy`?
   |
   = help: consider adding `extern crate clippy` to use the `clippy` crate

error[E0433]: failed to resolve: maybe a missing crate `rustdoc`?
   |
   |
LL | use rustdoc::a::b; //~ ERROR failed to resolve: maybe a missing crate `rustdoc`?
   |     ^^^^^^^ maybe a missing crate `rustdoc`?
   |
   = help: consider adding `extern crate rustdoc` to use the `rustdoc` crate
error[E0432]: unresolved import `rustdoc`
  --> /checkout/src/test/ui/imports/tool-mod-child.rs:4:5
   |
   |
LL | use rustdoc::a; //~ ERROR unresolved import `rustdoc`
   |     ^^^^^^^ maybe a missing crate `rustdoc`?
   |
   = help: consider adding `extern crate rustdoc` to use the `rustdoc` crate
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
For more information about an error, try `rustc --explain E0432`.
------------------------------------------


---- [ui] src/test/ui/keyword/extern/keyword-extern-as-identifier-use.rs stdout ----
diff of stderr:

14    |
15 LL | use extern::foo;
16    |     ^^^^^^ maybe a missing crate `r#extern`?
+    |
+    = help: consider adding `extern crate r#extern` to use the `r#extern` crate
18 error: aborting due to 2 previous errors
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword/extern/keyword-extern-as-identifier-use/keyword-extern-as-identifier-use.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args keyword/extern/keyword-extern-as-identifier-use.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/keyword/extern/keyword-extern-as-identifier-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword/extern/keyword-extern-as-identifier-use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword/extern/keyword-extern-as-identifier-use/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `extern`
   |
   |
LL | use extern::foo; //~ ERROR expected identifier, found keyword `extern`
   |
   |
help: escape `extern` to use it as an identifier
   |
LL | use r#extern::foo; //~ ERROR expected identifier, found keyword `extern`

error[E0432]: unresolved import `r#extern`
  --> /checkout/src/test/ui/keyword/extern/keyword-extern-as-identifier-use.rs:1:5
   |
   |
LL | use extern::foo; //~ ERROR expected identifier, found keyword `extern`
   |     ^^^^^^ maybe a missing crate `r#extern`?
   |
   = help: consider adding `extern crate r#extern` to use the `r#extern` crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/privacy/restricted/test.rs stdout ----
diff of stderr:

3    |
4 LL |     pub(in bad::path) mod m1 {}
5    |            ^^^ maybe a missing crate `bad`?
+    |
+    = help: consider adding `extern crate bad` to use the `bad` crate
7 error[E0742]: visibilities can only be restricted to ancestor modules
8   --> $DIR/test.rs:51:12



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/restricted/test/test.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/restricted/test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/restricted/test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/restricted/test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/restricted/test/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: maybe a missing crate `bad`?
   |
   |
LL |     pub(in bad::path) mod m1 {} //~ ERROR failed to resolve: maybe a missing crate `bad`?
   |            ^^^ maybe a missing crate `bad`?
   |
   = help: consider adding `extern crate bad` to use the `bad` crate
error[E0742]: visibilities can only be restricted to ancestor modules
  --> /checkout/src/test/ui/privacy/restricted/test.rs:51:12
   |
   |
LL |     pub(in foo) mod m2 {} //~ ERROR visibilities can only be restricted to ancestor modules


error[E0364]: `f` is private, and cannot be re-exported
   |
   |
LL |         pub(super) use foo::bar::f as g; //~ ERROR cannot be re-exported
   |
   |
note: consider marking `f` as `pub` in the imported module
   |
   |
LL |         pub(super) use foo::bar::f as g; //~ ERROR cannot be re-exported

error[E0603]: struct `Crate` is private
  --> /checkout/src/test/ui/privacy/restricted/test.rs:38:25
   |
   |
LL |     use pub_restricted::Crate; //~ ERROR private
   |                         ^^^^^ private struct
note: the struct `Crate` is defined here
  --> /checkout/src/test/ui/privacy/restricted/auxiliary/pub_restricted.rs:1:1
   |
LL | pub(crate) struct Crate;
LL | pub(crate) struct Crate;
   | ^^^^^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `f` is private
  --> /checkout/src/test/ui/privacy/restricted/test.rs:30:19
   |
LL |     use foo::bar::f; //~ ERROR private
   |                   ^ private function
note: the function `f` is defined here
  --> /checkout/src/test/ui/privacy/restricted/test.rs:8:9
   |
LL |         pub(super) fn f() {}
LL |         pub(super) fn f() {}
   |         ^^^^^^^^^^^^^^^^^

error[E0616]: field `x` of struct `S` is private
   |
   |
LL |     S::default().x; //~ ERROR private
   |                  ^ private field
error[E0624]: associated function `f` is private
  --> /checkout/src/test/ui/privacy/restricted/test.rs:32:18
   |
LL |             pub(super) fn f(&self) {}
LL |             pub(super) fn f(&self) {}
   |             ---------------------- private associated function defined here
...
LL |     S::default().f(); //~ ERROR private
   |                  ^ private associated function
error[E0624]: associated function `g` is private
  --> /checkout/src/test/ui/privacy/restricted/test.rs:33:8
   |
LL |             pub(super) fn g() {}
LL |             pub(super) fn g() {}
   |             ----------------- private associated function defined here
...
LL |     S::g(); //~ ERROR private
   |        ^ private associated function

error[E0616]: field `y` of struct `Universe` is private
   |
   |
LL |     let _ = u.y; //~ ERROR private
   |               ^ private field

error[E0616]: field `z` of struct `Universe` is private
   |
   |
LL |     let _ = u.z; //~ ERROR private
   |               ^ private field
error[E0624]: associated function `g` is private
  --> /checkout/src/test/ui/privacy/restricted/test.rs:45:7
   |
   |
LL |     u.g(); //~ ERROR private
   |       ^ private associated function
  ::: /checkout/src/test/ui/privacy/restricted/auxiliary/pub_restricted.rs:12:5
   |
   |
LL |     pub(crate) fn g(&self) {}


error[E0624]: associated function `h` is private
   |
   |
LL |     u.h(); //~ ERROR private
   |       ^ private associated function
  ::: /checkout/src/test/ui/privacy/restricted/auxiliary/pub_restricted.rs:13:5
   |
   |
LL |     pub(crate) fn h(&self) {}

error: aborting due to 12 previous errors

Some errors have detailed explanations: E0364, E0433, E0603, E0616, E0624, E0742.
