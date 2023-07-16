plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
    --> src/builder.rs:1109:54
     |
1109 |         let current_ptrs: [Self::Value; VAR_COUNT] = core::array::from_fn(
1110 |             ||{
     |             -- takes 0 arguments
     |
help: consider changing the closure to take and ignore the expected argument
help: consider changing the closure to take and ignore the expected argument
     |
1110 |             |_|{
     |             ~~~

error[E0308]: mismatched types
    --> src/builder.rs:1112:46
     |
1112 |                 let start = self.pointercast(start, self.type_i8p());
     |                                  ----------- ^^^^^ expected `RValue<'_>`, found `&RValue<'_>`
     |                                  arguments to this method are incorrect
     |
note: method defined here
    --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:204:8
    --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:204:8
     |
204  |     fn pointercast(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value;
help: consider dereferencing the borrow
     |
     |
1112 |                 let start = self.pointercast(*start, self.type_i8p());

error[E0308]: mismatched types
    --> src/builder.rs:1114:43
     |
     |
1114 |                 let offset = self.gcc_mul(addition, loop_i_val);
     |                                   ------- ^^^^^^^^ expected `RValue<'_>`, found `&RValue<'_>`
     |                                   arguments to this method are incorrect
     |
note: method defined here
    --> src/int.rs:189:12
    --> src/int.rs:189:12
     |
189  |     pub fn gcc_mul(&self, a: RValue<'gcc>, b: RValue<'gcc>) -> RValue<'gcc> {
help: consider dereferencing the borrow
     |
     |
1114 |                 let offset = self.gcc_mul(*addition, loop_i_val);

error[E0308]: mismatched types
    --> src/builder.rs:1119:31
     |
     |
1119 |         let next_i = self.add(loop_i, self.const_usize(1));
     |                           --- ^^^^^^ expected `RValue<'_>`, found `LValue<'_>`
     |                           arguments to this method are incorrect
     |
note: method defined here
    --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:86:8
    --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:86:8
     |
86   |     fn add(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;

Some errors have detailed explanations: E0308, E0593.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_gcc` (lib) due to 4 previous errors
