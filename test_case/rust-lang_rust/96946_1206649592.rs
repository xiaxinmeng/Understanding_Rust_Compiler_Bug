plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: arguments to this function are incorrect
    --> src/intrinsics/mod.rs:545:26
     |
545  |             fx.bcx.ins().band(ptr, mask);
     |                          ^^^^ ---  ---- expected struct `cranelift_codegen::ir::Value`, found struct `value_and_place::CValue`
     |                               |
     |                               expected struct `cranelift_codegen::ir::Value`, found struct `value_and_place::CValue`
note: associated function defined here
    --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/cranelift-codegen-98ed3bec4c044c17/out/inst_builder.rs:2496:8
     |
     |
2496 |     fn band(self, x: ir::Value, y: ir::Value) -> Value {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:28
