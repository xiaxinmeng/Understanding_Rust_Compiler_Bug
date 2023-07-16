plain
    Checking cranelift-frontend v0.83.0
    Checking cranelift-native v0.83.0
    Checking cranelift-object v0.83.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0004]: non-exhaustive patterns: `CustomSlice { .. }` not covered
    |
169 |     match const_val {
169 |     match const_val {
    |           ^^^^^^^^^ pattern `CustomSlice { .. }` not covered
note: `rustc_middle::mir::interpret::ConstValue` defined here
   --> /checkout/compiler/rustc_middle/src/mir/interpret/value.rs:41:5
    |
31  | / pub enum ConstValue<'tcx> {
31  | / pub enum ConstValue<'tcx> {
32  | |     /// Used only for types with `layout::abi::Scalar` ABI and ZSTs.
33  | |     ///
34  | |     /// Not using the enum `Value` to encode that this must not be `Uninit`.
...   |
41  | |     CustomSlice { data: ConstAllocation<'tcx>, length: usize },
...   |
50  | |     },
51  | | }
    | |_-
    | |_-
    = note: the matched value is of type `rustc_middle::mir::interpret::ConstValue`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
253 ~         }
253 ~         }
254 +         CustomSlice { .. } => todo!()

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:04:04
