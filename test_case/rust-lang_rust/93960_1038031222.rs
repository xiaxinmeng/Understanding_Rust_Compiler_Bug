plain
---- [ui] ui/consts/rustc-impl-const-stability.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rustc-impl-const-stability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rustc-impl-const-stability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rustc-impl-const-stability/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/rfc-2632-const-trait-impl/stability.rs stdout ----
diff of stderr:

1 error: `<Int as Add>::add` is not yet stable as a const fn
-   --> $DIR/stability.rs:33:5
3    |
4 LL |     Int(0) + Int(0);
5    |     ^^^^^^^^^^^^^^^


7    = help: const-stable functions can only call other const-stable functions
8 
9 error: trait implementations cannot be const stable yet
-   --> $DIR/stability.rs:11:1
+   --> $DIR/stability.rs:10:1
11    |
12 LL | / impl const std::ops::Sub for Int {
13 LL | |

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/stability/stability.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/stability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/stability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/stability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/stability/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `<Int as Add>::add` is not yet stable as a const fn
   |
LL |     Int(0) + Int(0);
   |     ^^^^^^^^^^^^^^^
   |
   |
   = help: const-stable functions can only call other const-stable functions

error: trait implementations cannot be const stable yet
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/stability.rs:10:1
   |
LL | / impl const std::ops::Sub for Int {
LL | |     //~^ ERROR trait implementations cannot be const stable yet
LL | |     type Output = Self;
LL | |
LL | |     }
LL | | }
   | |_^
   |
