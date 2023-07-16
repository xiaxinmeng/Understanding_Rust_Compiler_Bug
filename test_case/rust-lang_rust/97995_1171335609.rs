plain
........................................................................................ 12408/13133
........................................................................................ 12496/13133
........................................................................................ 12584/13133
........................................................................................ 12672/13133
.F.............................................F........................................ 12760/13133
........................................................................................ 12936/13133
........................................................................................ 13024/13133
..iii................................................................................... 13112/13133
.....................
.....................
failures:

---- [ui] src/test/ui/union/field_checks.rs stdout ----

1 error[E0740]: unions cannot contain fields that may need dropping
-   --> $DIR/field_checks.rs:25:5
+   --> $DIR/field_checks.rs:24:5
---
13 error[E0740]: unions cannot contain fields that may need dropping
-   --> $DIR/field_checks.rs:29:5
+   --> $DIR/field_checks.rs:28:5
15    |
16 LL |     a: std::cell::RefCell<i32>,

23    |        +++++++++++++++++++++++                       +
24 
25 error[E0740]: unions cannot contain fields that may need dropping
---
49 error[E0740]: unions cannot contain fields that may need dropping
-   --> $DIR/field_checks.rs:49:5
+   --> $DIR/field_checks.rs:48:5
51    |
52 LL |     nest: [U5; 0],


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/field_checks/field_checks.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/field_checks/field_checks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args union/field_checks.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/field_checks.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/field_checks" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/field_checks/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0740]: unions cannot contain fields that may need dropping
  --> /checkout/src/test/ui/union/field_checks.rs:24:5
   |
LL |     a: String, //~ ERROR unions cannot contain fields that may need dropping
   |
   |
   = note: a type is guaranteed not to need dropping when it implements `Copy`, or when it is the special `ManuallyDrop<_>` type
help: when the type does not implement `Copy`, wrap it inside a `ManuallyDrop<_>` and ensure it is manually dropped
   |
LL |     a: std::mem::ManuallyDrop<String>, //~ ERROR unions cannot contain fields that may need dropping

error[E0740]: unions cannot contain fields that may need dropping
  --> /checkout/src/test/ui/union/field_checks.rs:28:5
   |
   |
LL |     a: std::cell::RefCell<i32>, //~ ERROR unions cannot contain fields that may need dropping
   |
   |
   = note: a type is guaranteed not to need dropping when it implements `Copy`, or when it is the special `ManuallyDrop<_>` type
help: when the type does not implement `Copy`, wrap it inside a `ManuallyDrop<_>` and ensure it is manually dropped
   |
LL |     a: std::mem::ManuallyDrop<std::cell::RefCell<i32>>, //~ ERROR unions cannot contain fields that may need dropping

error[E0740]: unions cannot contain fields that may need dropping
  --> /checkout/src/test/ui/union/field_checks.rs:32:5
   |
   |
LL |     a: T, //~ ERROR unions cannot contain fields that may need dropping
   |
   |
   = note: a type is guaranteed not to need dropping when it implements `Copy`, or when it is the special `ManuallyDrop<_>` type
help: when the type does not implement `Copy`, wrap it inside a `ManuallyDrop<_>` and ensure it is manually dropped
   |
LL |     a: std::mem::ManuallyDrop<T>, //~ ERROR unions cannot contain fields that may need dropping
   |        +++++++++++++++++++++++ +
error[E0740]: unions cannot contain fields that may need dropping
  --> /checkout/src/test/ui/union/field_checks.rs:44:5
   |
   |
LL |     nest: U5, //~ ERROR unions cannot contain fields that may need dropping
   |
   |
   = note: a type is guaranteed not to need dropping when it implements `Copy`, or when it is the special `ManuallyDrop<_>` type
help: when the type does not implement `Copy`, wrap it inside a `ManuallyDrop<_>` and ensure it is manually dropped
   |
LL |     nest: std::mem::ManuallyDrop<U5>, //~ ERROR unions cannot contain fields that may need dropping
   |           +++++++++++++++++++++++  +
error[E0740]: unions cannot contain fields that may need dropping
  --> /checkout/src/test/ui/union/field_checks.rs:48:5
   |
   |
LL |     nest: [U5; 0], //~ ERROR unions cannot contain fields that may need dropping
   |
   |
   = note: a type is guaranteed not to need dropping when it implements `Copy`, or when it is the special `ManuallyDrop<_>` type
help: when the type does not implement `Copy`, wrap it inside a `ManuallyDrop<_>` and ensure it is manually dropped
   |
LL |     nest: std::mem::ManuallyDrop<[U5; 0]>, //~ ERROR unions cannot contain fields that may need dropping

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0740`.
For more information about this error, try `rustc --explain E0740`.
------------------------------------------


---- [ui] src/test/ui/union/union-nonrepresentable.rs stdout ----
diff of stderr:

1 error[E0072]: recursive type `U` has infinite size
-   --> $DIR/union-nonrepresentable.rs:2:1
3    |
4 LL | union U {
5    | ^^^^^^^ recursive type has infinite size

---
To only update this specific test, also pass `--test-args union/union-nonrepresentable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-nonrepresentable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-nonrepresentable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-nonrepresentable/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0072]: recursive type `U` has infinite size
   |
   |
LL | union U { //~ ERROR recursive type `U` has infinite size
   | ^^^^^^^ recursive type has infinite size
LL |     a: u8,
LL |     b: std::mem::ManuallyDrop<U>,
   |        ------------------------- recursive without indirection
   |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `U` representable
   |
LL |     b: Box<std::mem::ManuallyDrop<U>>,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0072`.
