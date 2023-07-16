plain
---- [ui] src/test/ui-fulldeps/uninit_mask.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/uninit_mask.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/uninit_mask/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/uninit_mask/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `rustc_middle::mir::interpret::InitMask`
   |
LL | use rustc_middle::mir::interpret::InitMask;
LL | use rustc_middle::mir::interpret::InitMask;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `InitMask` in `mir::interpret`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
