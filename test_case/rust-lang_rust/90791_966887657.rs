plain
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking object v0.25.3
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0201]: duplicate definitions with name `const_i32`:
    |
    |
150 | /     fn const_i32(&self, i: i32) -> RValue<'gcc> {
151 | |         self.const_int(self.type_i32(), i as i64)
152 | |     }
    | |_____- previous definition of `const_i32` here
153 | 
154 | /     fn const_i32(&self, i: i64) -> RValue<'gcc> {
155 | |         self.const_int(self.type_i64(), i)
    | |_____^ duplicate definition

For more information about this error, try `rustc --explain E0201`.
error: could not compile `rustc_codegen_gcc` due to previous error
