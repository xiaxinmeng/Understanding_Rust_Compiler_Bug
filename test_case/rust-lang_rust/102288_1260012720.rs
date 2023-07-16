plain

---- [ui] src/test/ui/suggestions/inner_type.rs stdout ----
diff of fixed:

38     //~^ ERROR no method named `some_mutable_method` found for struct `RwLock` in the current scope [E0599]
39     //~| HELP use `.write()` to mutably borrow the `Struct<u32>`, blocking the current thread until it can be acquired
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/inner_type/inner_type.fixed
To only update this specific test, also pass `--test-args suggestions/inner_type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/inner_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/inner_type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/inner_type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `method` found for struct `RefCell` in the current scope
   |
   |
LL |     other_item.method();
   |                ^^^^^^ method not found in `RefCell<Struct<u32>>`
   |
note: the method `method` exists on the type `Struct<u32>`
   |
LL |     pub fn method(&self) {}
   |     ^^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^^
help: use `.borrow()` to borrow the `Struct<u32>`, panicking if any outstanding mutable borrows exist.
   |
LL |     other_item.borrow().method();


error[E0599]: no method named `some_mutable_method` found for struct `RefCell` in the current scope
   |
   |
LL |     other_item.some_mutable_method();
   |                ^^^^^^^^^^^^^^^^^^^ method not found in `RefCell<Struct<u32>>`
   |
note: the method `some_mutable_method` exists on the type `Struct<u32>`
   |
LL |     pub fn some_mutable_method(&mut self) {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use `.borrow_mut()` to mutably borrow the `Struct<u32>`, panicking if any outstanding borrows exist.
   |
LL |     other_item.borrow_mut().some_mutable_method();


error[E0599]: no method named `method` found for struct `Mutex` in the current scope
   |
   |
LL |     another_item.method();
   |                  ^^^^^^ method not found in `Mutex<Struct<u32>>`
   |
note: the method `method` exists on the type `Struct<u32>`
   |
LL |     pub fn method(&self) {}
   |     ^^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^^
help: use `.lock()` to borrow the `Struct<u32>`, blocking the current thread until it can be acquired
   |
LL |     another_item.lock().unwrap().method();


error[E0599]: no method named `method` found for struct `RwLock` in the current scope
   |
   |
LL |     another_item.method();
   |                  ^^^^^^ method not found in `RwLock<Struct<u32>>`
   |
note: the method `method` exists on the type `Struct<u32>`
   |
LL |     pub fn method(&self) {}
   |     ^^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^^
help: use `.read()` to borrow the `Struct<u32>`, blocking the current thread until it can be acquired
   |
LL |     another_item.read().unwrap().method();


error[E0599]: no method named `some_mutable_method` found for struct `RwLock` in the current scope
   |
   |
LL |     another_item.some_mutable_method();
   |                  ^^^^^^^^^^^^^^^^^^^ method not found in `RwLock<Struct<u32>>`
   |
note: the method `some_mutable_method` exists on the type `Struct<u32>`
   |
LL |     pub fn some_mutable_method(&mut self) {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use `.write()` to mutably borrow the `Struct<u32>`, blocking the current thread until it can be acquired
   |
LL |     another_item.write().unwrap().some_mutable_method();

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0599`.
