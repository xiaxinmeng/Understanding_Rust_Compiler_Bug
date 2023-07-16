plain
[00:06:04]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:13]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:52]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:14]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:27] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::Variants` in the current scope
[00:08:27]     --> librustc/macros.rs:104:27
[00:08:27]      |
[00:08:27] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:08:27]      | 
[00:08:27]      | 
[00:08:27]     ::: librustc/ty/layout.rs:1999:1
[00:08:27]      |
[00:08:27] 1999 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:08:27] 2000 | |     variants,
[00:08:27] 2001 | |     fields,
[00:08:27] 2002 | |     abi,
[00:08:27] 2004 | |     align
[00:08:27] 2005 | | });
[00:08:27] 2005 | | });
[00:08:27]      | |___- in this macro invocation
[00:08:27]      |
[00:08:27]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:08:27]              `&rustc_target::abi::Variants : rustc_data_structures::stable_hasher::HashStable<_>`
[00:08:27] 
[00:08:27] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::FieldPlacement` in the current scope
[00:08:27]     --> librustc/macros.rs:104:27
[00:08:27]      |
[00:08:27] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:08:27]      | 
[00:08:27]      | 
[00:08:27]     ::: librustc/ty/layout.rs:1999:1
[00:08:27]      |
[00:08:27] 1999 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:08:27] 2000 | |     variants,
[00:08:27] 2001 | |     fields,
[00:08:27] 2002 | |     abi,
[00:08:27] 2004 | |     align
[00:08:27] 2005 | | });
[00:08:27] 2005 | | });
[00:08:27]      | |___- in this macro invocation
[00:08:27]      |
[00:08:27]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:08:27]              `&rustc_target::abi::FieldPlacement : rustc_data_structures::stable_hasher::HashStable<_>`
[00:08:27] 
[00:08:27] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::Abi` in the current scope
[00:08:27]     --> librustc/macros.rs:104:27
[00:08:27]      |
[00:08:27] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:08:27]      | 
[00:08:27]      | 
[00:08:27]     ::: librustc/ty/layout.rs:1999:1
[00:08:27]      |
[00:08:27] 1999 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:08:27] 2000 | |     variants,
[00:08:27] 2001 | |     fields,
[00:08:27] 2002 | |     abi,
[00:08:27] 2004 | |     align
[00:08:27] 2005 | | });
[00:08:27] 2005 | | });
[00:08:27]      | |___- in this macro invocation
[00:08:27]      |
[00:08:27]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:08:27]              `&rustc_target::abi::Abi : rustc_data_structures::stable_hasher::HashStable<_>`
[00:08:27] 
[00:08:43] error[E0277]: the trait bound `ty::layout::LayoutError<'_>: serialize::UseSpecializedEncodable` is not satisfied
[00:08:43]   --> librustc/mir/interpret/error.rs:33:17
[00:08:43]    |
[00:08:43] 33 | #[derive(Clone, RustcEncodable, RustcDecodable)]
[00:08:43]    |                 ^^^^^^^^^^^^^^ the trait `serialize::UseSpecializedEncodable` is not implemented for `ty::layout::LayoutError<'_>`
[00:08:43]    |
[00:08:43]    = note: required because of the requirements on the impl of `serialize::Encodable` for `ty::layout::LayoutError<'_>`
[00:08:43]    = note: required by `serialize::Encodable::encode`
[00:08:43] 
[00:08:43] error[E0277]: the trait bound `ty::layout::LayoutError<'_>: serialize::UseSpecializedDecodable` is not satisfied
[00:08:43]   --> librustc/mir/interpret/error.rs:33:33
[00:08:43]    |
[00:08:43] 33 | #[derive(Clone, RustcEncodable, RustcDecodable)]
[00:08:43]    |                                 ^^^^^^^^^^^^^^ the trait `serialize::UseSpecializedDecodable` is not implemented for `ty::layout::LayoutError<'_>`
[00:08:43]    |
[00:08:43]    = note: required because of the requirements on the impl of `serialize::Decodable` for `ty::layout::LayoutError<'_>`
[00:08:43]    = note: required by `serialize::Decodable::decode`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:410:27
[00:08:48]     |
[00:08:48]     |
[00:08:48] 410 |                 variants: Variants::Single { index: 0 },
[00:08:48]     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:48]     = note: expected type `rustc_target::abi::Variants`
[00:08:48]     = note: expected type `rustc_target::abi::Variants`
[00:08:48]                found type `ty::layout::Variants`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:411:25
[00:08:48]     |
[00:08:48]     |
[00:08:48] 411 |                   fields: FieldPlacement::Arbitrary {
[00:08:48]     |  _________________________^
[00:08:48] 412 | |                     offsets: vec![Size::from_bytes(0), b_offset],
[00:08:48] 413 | |                     memory_index: vec![0, 1]
[00:08:48] 414 | |                 },
[00:08:48]     | |_________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:48]     |
[00:08:48]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:48]                found type `ty::layout::FieldPlacement`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:415:22
[00:08:48]     |
[00:08:48]     |
[00:08:48] 415 |                 abi: Abi::ScalarPair(a, b),
[00:08:48]     |                      ^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]                found type `ty::layout::Abi`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:507:33
[00:08:48]     |
[00:08:48]     |
[00:08:48] 507 |                 if field.abi == Abi::Uninhabited {
[00:08:48]     |                                 ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]                found type `ty::layout::Abi`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:597:60
[00:08:48]     |
[00:08:48]     |
[00:08:48] 597 |                             details: &LayoutDetails { abi: Abi::Scalar(ref a), .. }, ..
[00:08:48]     |                                                            ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]                found type `ty::layout::Abi`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:599:60
[00:08:48]     |
[00:08:48]     |
[00:08:48] 599 |                             details: &LayoutDetails { abi: Abi::Scalar(ref b), .. }, ..
[00:08:48]     |                                                            ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]                found type `ty::layout::Abi`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:582:37
[00:08:48]     |
[00:08:48]     |
[00:08:48] 582 |                                     Abi::Scalar(_) | Abi::Vector { .. } if optimize => {
[00:08:48]     |                                     ^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]                found type `ty::layout::Abi`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:582:54
[00:08:48]     |
[00:08:48]     |
[00:08:48] 582 |                                     Abi::Scalar(_) | Abi::Vector { .. } if optimize => {
[00:08:48]     |                                                      ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]                found type `ty::layout::Abi`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:587:37
[00:08:48]     |
[00:08:48]     |
[00:08:48] 587 |                                     Abi::ScalarPair(..) => {
[00:08:48]     |                                     ^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]     = note: expected type `rustc_target::abi::Abi`
[00:08:48]                found type `ty::layout::Abi`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:583:47
[00:08:48]     |
[00:08:48]     |
[00:08:48] 583 |                                         abi = field.abi.clone();
[00:08:48]     |                                               ^^^^^^^^^^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:08:48]     |
[00:08:48]     = note: expected type `ty::layout::Abi`
[00:08:48] 
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:588:47
[00:08:48]     |
[00:08:48]     |
[00:08:48] 588 |                                         abi = field.abi.clone();
[00:08:48]     |                                               ^^^^^^^^^^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:08:48]     |
[00:08:48]     = note: expected type `ty::layout::Abi`
[00:08:48] 
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:609:33
[00:08:48]     |
[00:08:48]     |
[00:08:48] 609 | /                                 FieldPlacement::Arbitrary {
[00:08:48] 610 | |                                     ref offsets,
[00:08:48] 611 | |                                     ref memory_index
[00:08:48] 612 | |                                 } => {
[00:08:48]     | |_________________________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:48]     |
[00:08:48]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:48]                found type `ty::layout::FieldPlacement`
[00:08:48] error[E0308]: mismatched types
[00:08:48]    --> librustc/ty/layout.rs:624:39
[00:08:48]     |
[00:08:48]     |
[00:08:48] 624 |                                 abi = pair.abi;
[00:08:48]     |                                       ^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:08:48]     |
[00:08:48]     = note: expected type `ty::layout::Abi`
[00:08:48] 
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:634:27
[00:08:49]     |
[00:08:49]     |
[00:08:49] 634 |                 variants: Variants::Single { index: 0 },
[00:08:49]     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:49]     = note: expected type `rustc_target::abi::Variants`
[00:08:49]     = note: expected type `rustc_target::abi::Variants`
[00:08:49]                found type `ty::layout::Variants`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:635:25
[00:08:49]     |
[00:08:49]     |
[00:08:49] 635 |                   fields: FieldPlacement::Arbitrary {
[00:08:49] 636 | |                     offsets,
[00:08:49] 637 | |                     memory_index
[00:08:49] 638 | |                 },
[00:08:49] 638 | |                 },
[00:08:49]     | |_________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:49]     |
[00:08:49]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:49]                found type `ty::layout::FieldPlacement`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:639:17
[00:08:49]     |
[00:08:49] 639 |                 abi,
[00:08:49] 639 |                 abi,
[00:08:49]     |                 ^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]                found type `ty::layout::Abi`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:730:31
[00:08:49]     |
[00:08:49]     |
[00:08:49] 730 |                     variants: Variants::Single { index: 0 },
[00:08:49]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:49]     = note: expected type `rustc_target::abi::Variants`
[00:08:49]     = note: expected type `rustc_target::abi::Variants`
[00:08:49]                found type `ty::layout::Variants`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:731:29
[00:08:49]     |
[00:08:49]     |
[00:08:49] 731 |                       fields: FieldPlacement::Array {
[00:08:49]     |  _____________________________^
[00:08:49] 732 | |                         stride: element.size,
[00:08:49] 734 | |                     },
[00:08:49] 734 | |                     },
[00:08:49]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:49]     |
[00:08:49]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:49]                found type `ty::layout::FieldPlacement`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:735:26
[00:08:49]     |
[00:08:49]     |
[00:08:49] 735 |                     abi: Abi::Aggregate { sized: true },
[00:08:49]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]                found type `ty::layout::Abi`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:743:31
[00:08:49]     |
[00:08:49]     |
[00:08:49] 743 |                     variants: Variants::Single { index: 0 },
[00:08:49]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:49]     = note: expected type `rustc_target::abi::Variants`
[00:08:49]     = note: expected type `rustc_target::abi::Variants`
[00:08:49]                found type `ty::layout::Variants`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:744:29
[00:08:49]     |
[00:08:49]     |
[00:08:49] 744 |                       fields: FieldPlacement::Array {
[00:08:49]     |  _____________________________^
[00:08:49] 745 | |                         stride: element.size,
[00:08:49] 747 | |                     },
[00:08:49] 747 | |                     },
[00:08:49]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:49]     |
[00:08:49]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:49]                found type `ty::layout::FieldPlacement`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:748:26
[00:08:49]     |
[00:08:49]     |
[00:08:49] 748 |                     abi: Abi::Aggregate { sized: false },
[00:08:49]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]                found type `ty::layout::Abi`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:755:31
[00:08:49]     |
[00:08:49]     |
[00:08:49] 755 |                     variants: Variants::Single { index: 0 },
[00:08:49]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:49]     = note: expected type `rustc_target::abi::Variants`
[00:08:49]     = note: expected type `rustc_target::abi::Variants`
[00:08:49]                found type `ty::layout::Variants`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:756:29
[00:08:49]     |
[00:08:49]     |
[00:08:49] 756 |                       fields: FieldPlacement::Array {
[00:08:49]     |  _____________________________^
[00:08:49] 757 | |                         stride: Size::from_bytes(1),
[00:08:49] 759 | |                     },
[00:08:49] 759 | |                     },
[00:08:49]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:49]     |
[00:08:49]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:49]                found type `ty::layout::FieldPlacement`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:760:26
[00:08:49]     |
[00:08:49]     |
[00:08:49] 760 |                     abi: Abi::Aggregate { sized: false },
[00:08:49]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]                found type `ty::layout::Abi`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:774:21
[00:08:49]     |
[00:08:49]     |
[00:08:49] 774 |                     Abi::Aggregate { ref mut sized } => *sized = false,
[00:08:49]     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]                found type `ty::layout::Abi`
[00:08:49] error[E0308]: mismatched types
[00:08:49]    --> librustc/ty/layout.rs:812:21
[00:08:49]     |
[00:08:49]     |
[00:08:49] 812 |                     Abi::Scalar(ref scalar) => scalar.clone(),
[00:08:49]     |                     ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]     = note: expected type `rustc_target::abi::Abi`
[00:08:49]                found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:825:31
[00:08:50]     |
[00:08:50]     |
[00:08:50] 825 |                     variants: Variants::Single { index: 0 },
[00:08:50]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:50]     = note: expected type `rustc_target::abi::Variants`
[00:08:50]     = note: expected type `rustc_target::abi::Variants`
[00:08:50]                found type `ty::layout::Variants`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:826:29
[00:08:50]     |
[00:08:50]     |
[00:08:50] 826 |                       fields: FieldPlacement::Array {
[00:08:50]     |  _____________________________^
[00:08:50] 827 | |                         stride: element.size,
[00:08:50] 829 | |                     },
[00:08:50] 829 | |                     },
[00:08:50]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:50]     |
[00:08:50]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:50]                found type `ty::layout::FieldPlacement`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:830:26
[00:08:50]     |
[00:08:50]     |
[00:08:50] 830 |                       abi: Abi::Vector {
[00:08:50]     |  __________________________^
[00:08:50] 831 | |                         element: scalar,
[00:08:50] 833 | |                     },
[00:08:50] 833 | |                     },
[00:08:50]     | |_____________________^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]                found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:885:35
[00:08:50]     |
[00:08:50]     |
[00:08:50] 885 |                         variants: Variants::Single { index: 0 },
[00:08:50]     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:50]     = note: expected type `rustc_target::abi::Variants`
[00:08:50]     = note: expected type `rustc_target::abi::Variants`
[00:08:50]                found type `ty::layout::Variants`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:886:33
[00:08:50]     |
[00:08:50]     |
[00:08:50] 886 |                         fields: FieldPlacement::Union(variants[0].len()),
[00:08:50]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:50]     |
[00:08:50]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:50]                found type `ty::layout::FieldPlacement`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:887:30
[00:08:50]     |
[00:08:50]     |
[00:08:50] 887 |                         abi: Abi::Aggregate { sized: true },
[00:08:50]     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]                found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:895:61
[00:08:50]     |
[00:08:50]     |
[00:08:50] 895 |                         variants[v].iter().all(|f| f.abi != Abi::Uninhabited)
[00:08:50]     |                                                             ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]                found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:926:35
[00:08:50]     |
[00:08:50]     |
[00:08:50] 926 |                     st.variants = Variants::Single { index: v };
[00:08:50]     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:50]     = note: expected type `rustc_target::abi::Variants`
[00:08:50]     = note: expected type `rustc_target::abi::Variants`
[00:08:50]                found type `ty::layout::Variants`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:930:29
[00:08:50]     |
[00:08:50]     |
[00:08:50] 930 |                             Abi::Scalar(ref mut scalar) |
[00:08:50]     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]                found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:931:29
[00:08:50]     |
[00:08:50]     |
[00:08:50] 931 |                             Abi::ScalarPair(ref mut scalar, _) => {
[00:08:50]     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]                found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:955:59
[00:08:50]     |
[00:08:50]     |
[00:08:50] 955 |                         if fields.iter().any(|f| f.abi == Abi::Uninhabited) {
[00:08:50]     |                                                           ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]                found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:988:47
[00:08:50]     |
[00:08:50]     |
[00:08:50] 988 |                                 st.variants = Variants::Single { index: j };
[00:08:50]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:50]     = note: expected type `rustc_target::abi::Variants`
[00:08:50]     = note: expected type `rustc_target::abi::Variants`
[00:08:50]                found type `ty::layout::Variants`
[00:08:50] error[E0308]: mismatched types
[00:08:50]    --> librustc/ty/layout.rs:999:33
[00:08:50]     |
[00:08:50]     |
[00:08:50] 999 |                                 Abi::Scalar(_) => Abi::Scalar(niche.clone()),
[00:08:50]     |                                 ^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]     = note: expected type `rustc_target::abi::Abi`
[00:08:50]                found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]     --> librustc/ty/layout.rs:1000:33
[00:08:50]      |
[00:08:50]      |
[00:08:50] 1000 |                                 Abi::ScalarPair(ref first, ref second) => {
[00:08:50]      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:50]      = note: expected type `rustc_target::abi::Abi`
[00:08:50]      = note: expected type `rustc_target::abi::Abi`
[00:08:50]                 found type `ty::layout::Abi`
[00:08:50] error[E0308]: mismatched types
[00:08:50]     --> librustc/ty/layout.rs:1016:43
[00:08:50]      |
[00:08:50]      |
[00:08:50] 1016 |                                   variants: Variants::NicheFilling {
[00:08:50]      |  ___________________________________________^
[00:08:50] 1017 | |                                     dataful_variant: i,
[00:08:50] 1018 | |                                     niche_variants,
[00:08:50] 1019 | |                                     niche,
[00:08:50] 1020 | |                                     niche_start,
[00:08:50] 1021 | |                                     variants: st,
[00:08:50] 1022 | |                                 },
[00:08:50]      | |_________________________________^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:50]      = note: expected type `rustc_target::abi::Variants`
[00:08:50]      = note: expected type `rustc_target::abi::Variants`
[00:08:50]                 found type `ty::layout::Variants`
[00:08:50] error[E0308]: mismatched types
[00:08:50]     --> librustc/ty/layout.rs:1023:41
[00:08:50]      |
---
149616 ./.git/modules/src
149112 ./src/llvm-emscripten/test
144684 ./obj/build/bootstrap/debug/incremental
123716 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123712 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0nnr8rzdp-1cd6nir-1ujokh614sjd2
89684 ./src/llvm/test/CodeGen
82788 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
71000 ./.git/modules/src/tools
70944 ./obj/build/x86_64-unknown-linux-gnu/native
---
11744 ./src/doc
11000 ./.git/objects
10760 ./.git/objects/pack
10052 ./src/test/compile-fail
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
travis_time:end:00400a3c:start=1525253853370814361,finish=1525253853610249154,duration=239434793
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0287eb1b
