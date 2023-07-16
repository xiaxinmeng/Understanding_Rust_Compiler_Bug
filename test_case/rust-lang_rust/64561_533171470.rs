rust
error[E0308]: mismatched types
   --> src/libsyntax/feature_gate/check.rs:437:32
    |
437 |                 self.check_abi(bare_fn_ty.abi, ty.span);
    |                                ^^^^^^^^^^^^^^ expected enum `rustc_target::spec::abi::Abi`, found enum `std::option::Option`
    |
    = note: expected type `rustc_target::spec::abi::Abi`
               found type `std::option::Option<rustc_target::spec::abi::Abi>`

error[E0308]: mismatched types
   --> src/libsyntax/parse/parser/ty.rs:303:13
    |
303 |             abi,
    |             ^^^
    |             |
    |             expected enum `std::option::Option`, found enum `rustc_target::spec::abi::Abi`
    |             help: try using a variant of the expected type: `Some(abi)`
    |
    = note: expected type `std::option::Option<rustc_target::spec::abi::Abi>`
               found type `rustc_target::spec::abi::Abi`

error[E0308]: mismatched types
    --> src/libsyntax/print/pprust.rs:1006:34
     |
1006 |                 self.print_ty_fn(f.abi,
     |                                  ^^^^^ expected enum `rustc_target::spec::abi::Abi`, found enum `std::option::Option`
     |
     = note: expected type `rustc_target::spec::abi::Abi`
                found type `std::option::Option<rustc_target::spec::abi::Abi>`
