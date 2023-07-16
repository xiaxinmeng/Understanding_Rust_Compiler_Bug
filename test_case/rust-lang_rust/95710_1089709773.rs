plain

---- [ui] ui/cast/issue-88621.rs stdout ----
diff of stderr:

1 error[E0605]: non-primitive cast: `Kind2` as `u8`
-   --> $DIR/issue-88621.rs:11:13
3    |
3    |
4 LL |     let _ = Kind2::Foo() as u8;
5    |             ^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/issue-88621/issue-88621.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cast/issue-88621.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cast/issue-88621.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/issue-88621" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/issue-88621/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0605]: non-primitive cast: `Kind2` as `u8`
   |
   |
LL |     let _ = Kind2::Foo() as u8;
   |             ^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
error: aborting due to previous error

For more information about this error, try `rustc --explain E0605`.
------------------------------------------
------------------------------------------


---- [ui] ui/enum-discriminant/arbitrary_enum_discriminant-no-repr.rs stdout ----
diff of stderr:

1 error[E0732]: `#[repr(inttype)]` must be specified
-   --> $DIR/arbitrary_enum_discriminant-no-repr.rs:4:1
3    |
4 LL | / enum Enum {
5 LL | |

---
To only update this specific test, also pass `--test-args enum-discriminant/arbitrary_enum_discriminant-no-repr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enum-discriminant/arbitrary_enum_discriminant-no-repr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/arbitrary_enum_discriminant-no-repr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/arbitrary_enum_discriminant-no-repr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0732]: `#[repr(inttype)]` must be specified
   |
LL | / enum Enum {
LL | / enum Enum {
LL | | //~^ ERROR `#[repr(inttype)]` must be specified
LL | |   Unit = 1,
LL | |   Tuple() = 2,
LL | |   Struct{} = 3,
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0732`.
