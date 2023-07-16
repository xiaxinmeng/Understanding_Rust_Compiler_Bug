plain
failures:

---- [codegen] src/test/codegen/external-no-mangle-statics.rs#lib stdout ----

error in revision `lib`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/external-no-mangle-statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "lib" "-O" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-statics.lib" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-statics.lib/auxiliary" "--emit=llvm-ir"
stdout: none
--- stderr -------------------------------
error: `crate_type` within an `#![cfg_attr] attribute is deprecated`
  |
  |
7 | #![cfg_attr(lib, crate_type = "lib")]
  |
  |
  = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` on by default
  = note: for more information, see issue #91632 <https://github.com/rust-lang/rust/issues/91632>


warning: constant `HIDDEN` is never used
   |
   |
44 | const HIDDEN: () = {
   |
   = note: `#[warn(dead_code)]` on by default

error: aborting due to previous error; 1 warning emitted
error: aborting due to previous error; 1 warning emitted
------------------------------------------


---- [codegen] src/test/codegen/external-no-mangle-statics.rs#staticlib stdout ----

error in revision `staticlib`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/external-no-mangle-statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "staticlib" "-O" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-statics.staticlib" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-statics.staticlib/auxiliary" "--emit=llvm-ir"
stdout: none
--- stderr -------------------------------
error: `crate_type` within an `#![cfg_attr] attribute is deprecated`
  |
  |
8 | #![cfg_attr(staticlib, crate_type = "staticlib")]
  |
  |
  = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` on by default
  = note: for more information, see issue #91632 <https://github.com/rust-lang/rust/issues/91632>


warning: constant `HIDDEN` is never used
   |
   |
44 | const HIDDEN: () = {
   |
   = note: `#[warn(dead_code)]` on by default

error: aborting due to previous error; 1 warning emitted
