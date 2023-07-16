plain
    |
36  |   macro_rules! assert_eq {
    |   ---------------------- in this expansion of `$crate::assert_eq!` (#2)
...
53  |                   if !(*left_val == *right_val) {
    |                                     ^^^^^^^^^^ expected `(Size, AbiAndPrefAlign)`, found `(Size, Align)`
246 |   macro_rules! debug_assert_eq {
    |   ---------------------------- in this expansion of `debug_assert_eq!` (#1)
...
249 |               $crate::assert_eq!($($arg)*);
249 |               $crate::assert_eq!($($arg)*);
    |               ---------------------------- in this macro invocation (#2)
    |
   ::: compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs:185:13
    |
185 | /             debug_assert_eq!(
186 | |                 (data_layout.pointer_size, data_layout.pointer_align),
187 | |                 cx.size_and_align_of(ptr_type),
188 | |                 "ptr_type={}, pointee_type={}",
189 | |                 ptr_type,
190 | |                 pointee_type,
    | |_____________- in this macro invocation (#1)
    |
    |
    = note: expected tuple `(rustc_target::abi::Size, AbiAndPrefAlign)`
               found tuple `(rustc_target::abi::Size, Align)`

error[E0599]: no method named `bits` found for struct `AbiAndPrefAlign` in the current scope
   --> compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs:198:47
    |
198 |                     data_layout.pointer_align.bits() as u32,
    |                                               ^^^^ method not found in `AbiAndPrefAlign`
help: some of the expressions' fields have a method of the same name
    |
    |
198 |                     data_layout.pointer_align.abi.bits() as u32,
    |                                               ++++
198 |                     data_layout.pointer_align.pref.bits() as u32,

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_llvm` due to 2 previous errors
