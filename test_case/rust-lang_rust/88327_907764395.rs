plain
    Checking cranelift-frontend v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking cranelift-native v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking cranelift-object v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0507]: cannot move out of dereference of `rustc_target::abi::TyAndLayout<'_, &TyS<'_>>`
   |
   |
18 |     match layout.variants {
   |           ^^^^^^^^^^^^^^^ help: consider borrowing here: `&layout.variants`
...
45 |             tag_encoding: TagEncoding::Niche { dataful_variant, niche_variants, niche_start },
   |                                                                 |
   |                                                                 data moved here
   |                                                                 data moved here
   |                                                                 move occurs because `niche_variants` has type `RangeInclusive<rustc_target::abi::VariantIdx>`, which does not implement the `Copy` trait
For more information about this error, try `rustc --explain E0507`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:04:03
