plain
........................................................................................ 8712/13761
..........................i..ii......................................................... 8800/13761
.....ii................................................................................. 8888/13761
..............iiii...................................................................... 8976/13761
...................................F.F..................i............................... 9064/13761
........................................................................................ 9240/13761
.............................................................................i.......... 9328/13761
........................................................................................ 9416/13761
......................................................................................i. 9504/13761
---

---- [ui] src/test/ui/duplicate_entry_error.rs stdout ----
diff of stderr:

1 error[E0152]: found duplicate lang item `panic_impl`
3    |
3    |
- LL | fn panic_impl(info: &PanicInfo) -> ! {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | / fn panic_impl(info: &PanicInfo) -> ! {
+ LL | |
+ LL | |     loop {}
+ LL | | }
6    |
6    |
7    = note: the lang item is first defined in crate `std` (which `duplicate_entry_error` depends on)
8    = note: first definition in `std` loaded from SYSROOT/libstd-*.rlib

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/duplicate_entry_error.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args duplicate_entry_error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate_entry_error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0152]: found duplicate lang item `panic_impl`
   |
   |
LL | / fn panic_impl(info: &PanicInfo) -> ! {
LL | | //~^ ERROR: found duplicate lang item `panic_impl`
LL | |     loop {}
LL | | }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
   = note: the lang item is first defined in crate `std` (which `duplicate_entry_error` depends on)
   = note: first definition in `std` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-41c689254cd2f602.rlib
   = note: second definition in the local crate (`duplicate_entry_error`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0152`.
------------------------------------------
---
4 LL | struct Foo<T>(T);
-    | ^^^^^^^^^^^^^
+    | ^^^^^^^^^^^^^^^^^
6    |
7    = note: the lang item is first defined in crate `alloc` (which `std` depends on)
8    = note: first definition in `alloc` loaded from SYSROOT/liballoc-*.rlib

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152/E0152.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0152.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0152.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0152]: found duplicate lang item `owned_box`
   |
   |
LL | struct Foo<T>(T); //~ ERROR E0152
   |
   |
   = note: the lang item is first defined in crate `alloc` (which `std` depends on)
   = note: first definition in `alloc` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-3ada688740511823.rlib
   = note: second definition in the local crate (`E0152`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0152`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/panic-handler/panic-handler-duplicate.rs stdout ----
diff of stderr:

1 error[E0152]: found duplicate lang item `panic_impl`
3    |
3    |
- LL | fn panic2(info: &PanicInfo) -> ! {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | / fn panic2(info: &PanicInfo) -> ! {
+ LL | |     loop {}
+ LL | | }
6    |
7 note: the lang item is first defined here
8   --> $DIR/panic-handler-duplicate.rs:10:1


9    |
- LL | fn panic(info: &PanicInfo) -> ! {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | / fn panic(info: &PanicInfo) -> ! {
+ LL | |     loop {}
+ LL | | }
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args panic-handler/panic-handler-duplicate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-duplicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0152]: found duplicate lang item `panic_impl`
   |
   |
LL | / fn panic2(info: &PanicInfo) -> ! { //~ ERROR found duplicate lang item `panic_impl`
LL | |     loop {}
LL | | }
   |
note: the lang item is first defined here
  --> /checkout/src/test/ui/panic-handler/panic-handler-duplicate.rs:10:1
   |
   |
LL | / fn panic(info: &PanicInfo) -> ! {
LL | |     loop {}
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0152`.
For more information about this error, try `rustc --explain E0152`.
------------------------------------------


---- [ui] src/test/ui/panic-handler/panic-handler-std.rs stdout ----
diff of stderr:

1 error[E0152]: found duplicate lang item `panic_impl`
3    |
3    |
- LL | fn panic(info: PanicInfo) -> ! {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | / fn panic(info: PanicInfo) -> ! {
+ LL | |     loop {}
+ LL | | }
6    |
6    |
7    = note: the lang item is first defined in crate `std` (which `panic_handler_std` depends on)
8    = note: first definition in `std` loaded from SYSROOT/libstd-*.rlib

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/panic-handler-std.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panic-handler/panic-handler-std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-std.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0152]: found duplicate lang item `panic_impl`
   |
   |
LL | / fn panic(info: PanicInfo) -> ! {
LL | |     loop {}
LL | | }
   |
   |
   = note: the lang item is first defined in crate `std` (which `panic_handler_std` depends on)
   = note: first definition in `std` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-41c689254cd2f602.rlib
   = note: second definition in the local crate (`panic_handler_std`)

error: argument should be `&PanicInfo`
   |
   |
LL | fn panic(info: PanicInfo) -> ! {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0152`.
---
diff of stderr:

22   --> $DIR/issue-102989.rs:5:1
23    |
24 LL | trait Sized { }
+    | ^^^^^^^^^^^^^^^
26    |
26    |
27    = note: the lang item is first defined in crate `core` (which `std` depends on)
28    = note: first definition in `core` loaded from SYSROOT/libcore-*.rlib

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-102989/issue-102989.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-102989.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-102989.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-102989" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-102989/auxiliary"
stdout: none
--- stderr -------------------------------
error: `self` parameter is only allowed in associated functions
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
   |               ^^^^ not semantically valid as function parameter
   |
   = note: associated functions are those in `impl` or `trait` definitions
error[E0412]: cannot find type `Struct` in this scope
  --> /checkout/src/test/ui/traits/issue-102989.rs:7:22
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {

error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/traits/issue-102989.rs:11:13
   |
   |
LL |     let x = x << 1;
   |             ^ help: a local variable with a similar name exists: `f`

error[E0152]: found duplicate lang item `sized`
   |
   |
LL | trait Sized { } //~ ERROR found duplicate lang item `sized`
   |
   |
   = note: the lang item is first defined in crate `core` (which `std` depends on)
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`issue_102989`)
error[E0277]: the size for values of type `{integer}` cannot be known at compilation time
  --> /checkout/src/test/ui/traits/issue-102989.rs:11:15
   |
LL |     let x = x << 1;
LL |     let x = x << 1;
   |               ^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `{integer}`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/traits/issue-102989.rs:7:42
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
   |    ----------                            ^^^^ expected `&u32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
note: consider returning one of these bindings
  --> /checkout/src/test/ui/traits/issue-102989.rs:7:30
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
...
LL |     let x = x << 1;
   |         ^

