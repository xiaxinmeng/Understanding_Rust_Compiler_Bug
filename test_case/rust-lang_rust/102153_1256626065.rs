plain

---- [ui] src/test/ui/extern-flag/empty-extern-arg.rs stdout ----
diff of stderr:

7    = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
8    = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
- error: aborting due to 3 previous errors
- error: aborting due to 3 previous errors
+ error: language item required, but not found: `eh_catch_typeinfo`
+    |
+    = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_catch_typeinfo` is defined in the standard library
+    = help: you may be able to compile for a target that doesn't need `eh_catch_typeinfo`, specify a target with `--target` or in `.cargo/config`
+ error: aborting due to 4 previous errors
11 
12 

---
To only update this specific test, also pass `--test-args extern-flag/empty-extern-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern-flag/empty-extern-arg.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/empty-extern-arg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--extern" "std=" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/empty-extern-arg/auxiliary"
stdout: none
--- stderr -------------------------------
error: extern location for std does not exist: 

error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
   |
   = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
   = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`

error: language item required, but not found: `eh_catch_typeinfo`
   |
   = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_catch_typeinfo` is defined in the standard library
   = help: you may be able to compile for a target that doesn't need `eh_catch_typeinfo`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/panic-handler/weak-lang-item.rs stdout ----
diff of stderr:

17    = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
18    = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
- error: aborting due to 3 previous errors
- error: aborting due to 3 previous errors
+ error: language item required, but not found: `eh_catch_typeinfo`
+    |
+    = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_catch_typeinfo` is defined in the standard library
+    = help: you may be able to compile for a target that doesn't need `eh_catch_typeinfo`, specify a target with `--target` or in `.cargo/config`
+ error: aborting due to 4 previous errors
21 
22 For more information about this error, try `rustc --explain E0259`.
23 
---
To only update this specific test, also pass `--test-args panic-handler/weak-lang-item.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/weak-lang-item.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/weak-lang-item" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/weak-lang-item/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0259]: the name `core` is defined multiple times
   |
LL | extern crate core;
   | ^^^^^^^^^^^^^^^^^^ `core` reimported here
   |
   |
   = note: `core` must be defined only once in the type namespace of this module
help: you can use `as` to change the binding name of the import
LL | extern crate core as other_core;
   |


error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
   |
   = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
   = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`

error: language item required, but not found: `eh_catch_typeinfo`
   |
   = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_catch_typeinfo` is defined in the standard library
   = help: you may be able to compile for a target that doesn't need `eh_catch_typeinfo`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0259`.
------------------------------------------
