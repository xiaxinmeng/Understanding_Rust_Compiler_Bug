plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `arch` found for struct `rustc_target::spec::Target` in the current scope
    |
    |
862 |         match self.sess().target.arch() {
    |                                  ^^^^-- help: remove the arguments
    |                                  field, not a method

error[E0308]: mismatched types
   --> compiler/rustc_codegen_llvm/src/intrinsic.rs:333:25
   --> compiler/rustc_codegen_llvm/src/intrinsic.rs:333:25
    |
332 |                     match self.cx.sess().target.arch {
    |                           -------------------------- this expression has type `std::string::String`
333 |                         "avr" => self.icmp(IntPredicate::IntEQ, cmp, self.const_i16(0)),
    |                         ^^^^^ expected struct `std::string::String`, found `&str`
error[E0308]: mismatched types
   --> compiler/rustc_codegen_llvm/src/intrinsic.rs:334:25
    |
    |
332 |                     match self.cx.sess().target.arch {
    |                           -------------------------- this expression has type `std::string::String`
333 |                         "avr" => self.icmp(IntPredicate::IntEQ, cmp, self.const_i16(0)),
334 |                         "msp430" => self.icmp(IntPredicate::IntEQ, cmp, self.const_i16(0)),
    |                         ^^^^^^^^ expected struct `std::string::String`, found `&str`
error[E0308]: mismatched types
   --> compiler/rustc_codegen_llvm/src/intrinsic.rs:335:25
    |
    |
332 |                     match self.cx.sess().target.arch {
...
...
335 |                         "_" => self.icmp(IntPredicate::IntEQ, cmp, self.const_i32(0)),
    |                         ^^^ expected struct `std::string::String`, found `&str`
Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_llvm` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
