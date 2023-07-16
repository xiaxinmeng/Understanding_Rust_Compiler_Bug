plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: mismatched types
  --> src/abi/pass_mode.rs:97:60
   |
97 |                     AbiParam::new(scalar_to_clif_type(tcx, scalar)),
   |                                                            |
   |                                                            |
   |                                                            expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
   |                                                            help: consider dereferencing the borrow: `*scalar`
error[E0308]: mismatched types
   --> src/abi/pass_mode.rs:144:72
    |
    |
144 |                     (None, vec![AbiParam::new(scalar_to_clif_type(tcx, scalar))])
    |                                                                        |
    |                                                                        |
    |                                                                        expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                                        help: consider dereferencing the borrow: `*scalar`
error[E0308]: mismatched types
   --> src/value_and_place.rs:148:71
    |
    |
148 |                 let b_offset = scalar_pair_calculate_b_offset(fx.tcx, a_scalar, b_scalar);
    |                                                                       |
    |                                                                       |
    |                                                                       expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                                       help: consider dereferencing the borrow: `*a_scalar`
error[E0308]: mismatched types
   --> src/value_and_place.rs:148:81
    |
    |
148 |                 let b_offset = scalar_pair_calculate_b_offset(fx.tcx, a_scalar, b_scalar);
    |                                                                                 |
    |                                                                                 |
    |                                                                                 expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                                                 help: consider dereferencing the borrow: `*b_scalar`
error[E0308]: mismatched types
   --> src/value_and_place.rs:149:60
    |
    |
149 |                 let clif_ty1 = scalar_to_clif_type(fx.tcx, a_scalar);
    |                                                            |
    |                                                            |
    |                                                            expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                            help: consider dereferencing the borrow: `*a_scalar`
error[E0308]: mismatched types
   --> src/value_and_place.rs:150:60
    |
    |
150 |                 let clif_ty2 = scalar_to_clif_type(fx.tcx, b_scalar);
    |                                                            |
    |                                                            |
    |                                                            expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                            help: consider dereferencing the borrow: `*b_scalar`
error[E0308]: mismatched types
   --> src/value_and_place.rs:563:71
    |
    |
563 |                 let b_offset = scalar_pair_calculate_b_offset(fx.tcx, a_scalar, b_scalar);
    |                                                                       |
    |                                                                       |
    |                                                                       expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                                       help: consider dereferencing the borrow: `*a_scalar`
error[E0308]: mismatched types
   --> src/value_and_place.rs:563:81
    |
    |
563 |                 let b_offset = scalar_pair_calculate_b_offset(fx.tcx, a_scalar, b_scalar);
    |                                                                                 |
    |                                                                                 |
    |                                                                                 expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                                                 help: consider dereferencing the borrow: `*b_scalar`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to 8 previous errors
Build completed unsuccessfully in 0:03:28
