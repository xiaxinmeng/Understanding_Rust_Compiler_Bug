plain
[00:05:22]    Compiling backtrace v0.3.6
[00:05:29]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:52]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:10]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:20] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::Variants` in the current scope
[00:07:20]     --> librustc/macros.rs:104:27
[00:07:20]      |
[00:07:20] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:07:20]      | 
[00:07:20]      | 
[00:07:20]     ::: librustc/ty/layout.rs:1999:1
[00:07:20]      |
[00:07:20] 1999 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:07:20] 2000 | |     variants,
[00:07:20] 2001 | |     fields,
[00:07:20] 2002 | |     abi,
[00:07:20] 2004 | |     align
[00:07:20] 2005 | | });
[00:07:20] 2005 | | });
[00:07:20]      | |___- in this macro invocation
[00:07:20]      |
[00:07:20]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:07:20]              `&rustc_target::abi::Variants : rustc_data_structures::stable_hasher::HashStable<_>`
[00:07:20] 
[00:07:20] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::FieldPlacement` in the current scope
[00:07:20]     --> librustc/macros.rs:104:27
[00:07:20]      |
[00:07:20] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:07:20]      | 
[00:07:20]      | 
[00:07:20]     ::: librustc/ty/layout.rs:1999:1
[00:07:20]      |
[00:07:20] 1999 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:07:20] 2000 | |     variants,
[00:07:20] 2001 | |     fields,
[00:07:20] 2002 | |     abi,
[00:07:20] 2004 | |     align
[00:07:20] 2005 | | });
[00:07:20] 2005 | | });
[00:07:20]      | |___- in this macro invocation
[00:07:20]      |
[00:07:20]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:07:20]              `&rustc_target::abi::FieldPlacement : rustc_data_structures::stable_hasher::HashStable<_>`
[00:07:20] 
[00:07:20] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::Abi` in the current scope
[00:07:20]     --> librustc/macros.rs:104:27
[00:07:20]      |
[00:07:20] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:07:20]      | 
[00:07:20]      | 
[00:07:20]     ::: librustc/ty/layout.rs:1999:1
[00:07:20]      |
[00:07:20] 1999 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:07:20] 2000 | |     variants,
[00:07:20] 2001 | |     fields,
[00:07:20] 2002 | |     abi,
[00:07:20] 2004 | |     align
[00:07:20] 2005 | | });
[00:07:20] 2005 | | });
[00:07:20]      | |___- in this macro invocation
[00:07:20]      |
[00:07:20]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:07:20]              `&rustc_target::abi::Abi : rustc_data_structures::stable_hasher::HashStable<_>`
[00:07:20] 
[00:07:33] error[E0277]: the trait bound `ty::layout::LayoutError<'_>: serialize::UseSpecializedEncodable` is not satisfied
[00:07:33]   --> librustc/mir/interpret/error.rs:33:17
[00:07:33]    |
[00:07:33] 33 | #[derive(Clone, RustcEncodable, RustcDecodable)]
[00:07:33]    |                 ^^^^^^^^^^^^^^ the trait `serialize::UseSpecializedEncodable` is not implemented for `ty::layout::LayoutError<'_>`
[00:07:33]    |
[00:07:33]    = note: required because of the requirements on the impl of `serialize::Encodable` for `ty::layout::LayoutError<'_>`
[00:07:33]    = note: required by `serialize::Encodable::encode`
[00:07:33] 
[00:07:33] error[E0277]: the trait bound `ty::layout::LayoutError<'_>: serialize::UseSpecializedDecodable` is not satisfied
[00:07:33]   --> librustc/mir/interpret/error.rs:33:33
[00:07:33]    |
[00:07:33] 33 | #[derive(Clone, RustcEncodable, RustcDecodable)]
[00:07:33]    |                                 ^^^^^^^^^^^^^^ the trait `serialize::UseSpecializedDecodable` is not implemented for `ty::layout::LayoutError<'_>`
[00:07:33]    |
[00:07:33]    = note: required because of the requirements on the impl of `serialize::Decodable` for `ty::layout::LayoutError<'_>`
[00:07:33]    = note: required by `serialize::Decodable::decode`
[00:07:37] error[E0308]: mismatched types
[00:07:37]    --> librustc/ty/layout.rs:410:27
[00:07:37]     |
[00:07:37]     |
[00:07:37] 410 |                 variants: Variants::Single { index: 0 },
[00:07:37]     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:37]     = note: expected type `rustc_target::abi::Variants`
[00:07:37]     = note: expected type `rustc_target::abi::Variants`
[00:07:37]                found type `ty::layout::Variants`
[00:07:37] error[E0308]: mismatched types
[00:07:37]    --> librustc/ty/layout.rs:411:25
[00:07:37]     |
[00:07:37]     |
[00:07:37] 411 |                   fields: FieldPlacement::Arbitrary {
[00:07:37]     |  _________________________^
[00:07:37] 412 | |                     offsets: vec![Size::from_bytes(0), b_offset],
[00:07:37] 413 | |                     memory_index: vec![0, 1]
[00:07:37] 414 | |                 },
[00:07:37]     | |_________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:07:37]     |
[00:07:37]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:07:37]                found type `ty::layout::FieldPlacement`
[00:07:37] error[E0308]: mismatched types
[00:07:37]    --> librustc/ty/layout.rs:415:22
[00:07:37]     |
[00:07:37]     |
[00:07:37] 415 |                 abi: Abi::ScalarPair(a, b),
[00:07:37]     |                      ^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:37]     = note: expected type `rustc_target::abi::Abi`
[00:07:37]     = note: expected type `rustc_target::abi::Abi`
[00:07:37]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:507:33
[00:07:38]     |
[00:07:38]     |
[00:07:38] 507 |                 if field.abi == Abi::Uninhabited {
[00:07:38]     |                                 ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:597:60
[00:07:38]     |
[00:07:38]     |
[00:07:38] 597 |                             details: &LayoutDetails { abi: Abi::Scalar(ref a), .. }, ..
[00:07:38]     |                                                            ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:599:60
[00:07:38]     |
[00:07:38]     |
[00:07:38] 599 |                             details: &LayoutDetails { abi: Abi::Scalar(ref b), .. }, ..
[00:07:38]     |                                                            ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:582:37
[00:07:38]     |
[00:07:38]     |
[00:07:38] 582 |                                     Abi::Scalar(_) | Abi::Vector { .. } if optimize => {
[00:07:38]     |                                     ^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:582:54
[00:07:38]     |
[00:07:38]     |
[00:07:38] 582 |                                     Abi::Scalar(_) | Abi::Vector { .. } if optimize => {
[00:07:38]     |                                                      ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:587:37
[00:07:38]     |
[00:07:38]     |
[00:07:38] 587 |                                     Abi::ScalarPair(..) => {
[00:07:38]     |                                     ^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:583:47
[00:07:38]     |
[00:07:38]     |
[00:07:38] 583 |                                         abi = field.abi.clone();
[00:07:38]     |                                               ^^^^^^^^^^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:07:38]     |
[00:07:38]     = note: expected type `ty::layout::Abi`
[00:07:38] 
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:588:47
[00:07:38]     |
[00:07:38]     |
[00:07:38] 588 |                                         abi = field.abi.clone();
[00:07:38]     |                                               ^^^^^^^^^^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:07:38]     |
[00:07:38]     = note: expected type `ty::layout::Abi`
[00:07:38] 
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:609:33
[00:07:38]     |
[00:07:38]     |
[00:07:38] 609 | /                                 FieldPlacement::Arbitrary {
[00:07:38] 610 | |                                     ref offsets,
[00:07:38] 611 | |                                     ref memory_index
[00:07:38] 612 | |                                 } => {
[00:07:38]     | |_________________________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:07:38]     |
[00:07:38]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:07:38]                found type `ty::layout::FieldPlacement`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:624:39
[00:07:38]     |
[00:07:38]     |
[00:07:38] 624 |                                 abi = pair.abi;
[00:07:38]     |                                       ^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:07:38]     |
[00:07:38]     = note: expected type `ty::layout::Abi`
[00:07:38] 
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:634:27
[00:07:38]     |
[00:07:38]     |
[00:07:38] 634 |                 variants: Variants::Single { index: 0 },
[00:07:38]     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:38]     = note: expected type `rustc_target::abi::Variants`
[00:07:38]     = note: expected type `rustc_target::abi::Variants`
[00:07:38]                found type `ty::layout::Variants`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:635:25
[00:07:38]     |
[00:07:38]     |
[00:07:38] 635 |                   fields: FieldPlacement::Arbitrary {
[00:07:38] 636 | |                     offsets,
[00:07:38] 637 | |                     memory_index
[00:07:38] 638 | |                 },
[00:07:38] 638 | |                 },
[00:07:38]     | |_________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:07:38]     |
[00:07:38]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:07:38]                found type `ty::layout::FieldPlacement`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:639:17
[00:07:38]     |
[00:07:38] 639 |                 abi,
[00:07:38] 639 |                 abi,
[00:07:38]     |                 ^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:730:31
[00:07:38]     |
[00:07:38]     |
[00:07:38] 730 |                     variants: Variants::Single { index: 0 },
[00:07:38]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:38]     = note: expected type `rustc_target::abi::Variants`
[00:07:38]     = note: expected type `rustc_target::abi::Variants`
[00:07:38]                found type `ty::layout::Variants`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:731:29
[00:07:38]     |
[00:07:38]     |
[00:07:38] 731 |                       fields: FieldPlacement::Array {
[00:07:38]     |  _____________________________^
[00:07:38] 732 | |                         stride: element.size,
[00:07:38] 734 | |                     },
[00:07:38] 734 | |                     },
[00:07:38]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:07:38]     |
[00:07:38]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:07:38]                found type `ty::layout::FieldPlacement`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:735:26
[00:07:38]     |
[00:07:38]     |
[00:07:38] 735 |                     abi: Abi::Aggregate { sized: true },
[00:07:38]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:743:31
[00:07:38]     |
[00:07:38]     |
[00:07:38] 743 |                     variants: Variants::Single { index: 0 },
[00:07:38]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:38]     = note: expected type `rustc_target::abi::Variants`
[00:07:38]     = note: expected type `rustc_target::abi::Variants`
[00:07:38]                found type `ty::layout::Variants`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:744:29
[00:07:38]     |
[00:07:38]     |
[00:07:38] 744 |                       fields: FieldPlacement::Array {
[00:07:38]     |  _____________________________^
[00:07:38] 745 | |                         stride: element.size,
[00:07:38] 747 | |                     },
[00:07:38] 747 | |                     },
[00:07:38]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:07:38]     |
[00:07:38]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:07:38]                found type `ty::layout::FieldPlacement`
[00:07:38] error[E0308]: mismatched types
[00:07:38]    --> librustc/ty/layout.rs:748:26
[00:07:38]     |
[00:07:38]     |
[00:07:38] 748 |                     abi: Abi::Aggregate { sized: false },
[00:07:38]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]     = note: expected type `rustc_target::abi::Abi`
[00:07:38]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:755:31
[00:07:39]     |
[00:07:39]     |
[00:07:39] 755 |                     variants: Variants::Single { index: 0 },
[00:07:39]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]                found type `ty::layout::Variants`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:756:29
[00:07:39]     |
[00:07:39]     |
[00:07:39] 756 |                       fields: FieldPlacement::Array {
[00:07:39]     |  _____________________________^
[00:07:39] 757 | |                         stride: Size::from_bytes(1),
[00:07:39] 759 | |                     },
[00:07:39] 759 | |                     },
[00:07:39]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:07:39]     |
[00:07:39]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:07:39]                found type `ty::layout::FieldPlacement`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:760:26
[00:07:39]     |
[00:07:39]     |
[00:07:39] 760 |                     abi: Abi::Aggregate { sized: false },
[00:07:39]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:774:21
[00:07:39]     |
[00:07:39]     |
[00:07:39] 774 |                     Abi::Aggregate { ref mut sized } => *sized = false,
[00:07:39]     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:812:21
[00:07:39]     |
[00:07:39]     |
[00:07:39] 812 |                     Abi::Scalar(ref scalar) => scalar.clone(),
[00:07:39]     |                     ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:825:31
[00:07:39]     |
[00:07:39]     |
[00:07:39] 825 |                     variants: Variants::Single { index: 0 },
[00:07:39]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]                found type `ty::layout::Variants`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:826:29
[00:07:39]     |
[00:07:39]     |
[00:07:39] 826 |                       fields: FieldPlacement::Array {
[00:07:39]     |  _____________________________^
[00:07:39] 827 | |                         stride: element.size,
[00:07:39] 829 | |                     },
[00:07:39] 829 | |                     },
[00:07:39]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:07:39]     |
[00:07:39]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:07:39]                found type `ty::layout::FieldPlacement`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:830:26
[00:07:39]     |
[00:07:39]     |
[00:07:39] 830 |                       abi: Abi::Vector {
[00:07:39]     |  __________________________^
[00:07:39] 831 | |                         element: scalar,
[00:07:39] 833 | |                     },
[00:07:39] 833 | |                     },
[00:07:39]     | |_____________________^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:885:35
[00:07:39]     |
[00:07:39]     |
[00:07:39] 885 |                         variants: Variants::Single { index: 0 },
[00:07:39]     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]                found type `ty::layout::Variants`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:886:33
[00:07:39]     |
[00:07:39]     |
[00:07:39] 886 |                         fields: FieldPlacement::Union(variants[0].len()),
[00:07:39]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:07:39]     |
[00:07:39]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:07:39]                found type `ty::layout::FieldPlacement`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:887:30
[00:07:39]     |
[00:07:39]     |
[00:07:39] 887 |                         abi: Abi::Aggregate { sized: true },
[00:07:39]     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:895:61
[00:07:39]     |
[00:07:39]     |
[00:07:39] 895 |                         variants[v].iter().all(|f| f.abi != Abi::Uninhabited)
[00:07:39]     |                                                             ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:926:35
[00:07:39]     |
[00:07:39]     |
[00:07:39] 926 |                     st.variants = Variants::Single { index: v };
[00:07:39]     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]                found type `ty::layout::Variants`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:930:29
[00:07:39]     |
[00:07:39]     |
[00:07:39] 930 |                             Abi::Scalar(ref mut scalar) |
[00:07:39]     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:931:29
[00:07:39]     |
[00:07:39]     |
[00:07:39] 931 |                             Abi::ScalarPair(ref mut scalar, _) => {
[00:07:39]     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:955:59
[00:07:39]     |
[00:07:39]     |
[00:07:39] 955 |                         if fields.iter().any(|f| f.abi == Abi::Uninhabited) {
[00:07:39]     |                                                           ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:988:47
[00:07:39]     |
[00:07:39]     |
[00:07:39] 988 |                                 st.variants = Variants::Single { index: j };
[00:07:39]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]     = note: expected type `rustc_target::abi::Variants`
[00:07:39]                found type `ty::layout::Variants`
[00:07:39] error[E0308]: mismatched types
[00:07:39]    --> librustc/ty/layout.rs:999:33
[00:07:39]     |
[00:07:39]     |
[00:07:39] 999 |                                 Abi::Scalar(_) => Abi::Scalar(niche.clone()),
[00:07:39]     |                                 ^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]     = note: expected type `rustc_target::abi::Abi`
[00:07:39]                found type `ty::layout::Abi`
[00:07:39] error[E0308]: mismatched types
[00:07:39]     --> librustc/ty/layout.rs:1000:33
[00:07:39]      |
[00:07:39]      |
[00:07:39] 1000 |                                 Abi::ScalarPair(ref first, ref second) => {
[00:07:39]      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:07:39]      = note: expected type `rustc_target::abi::Abi`
[00:07:39]      = note: expected type `rustc_target::abi::Abi`
[00:07:39]                 found type `ty::layout::Abi`
[00:07:40] error[E0308]: mismatched types
[00:07:40]     --> librustc/ty/layout.rs:1016:43
[00:07:40]      |
[00:07:40]      |
[00:07:40] 1016 |                                   variants: Variants::NicheFilling {
[00:07:40]      |  ___________________________________________^
[00:07:40] 1017 | |                                     dataful_variant: i,
[00:07:40] 1018 | |                                     niche_variants,
[00:07:40] 1019 | |                                     niche,
[00:07:40] 1020 | |                                     niche_start,
[00:07:40] 1021 | |                                     variants: st,
[00:07:40] 1022 | |                                 },
[00:07:40]      | |_________________________________^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:07:40]      = note: expected type `rustc_target::abi::Variants`
[00:07:40]      = note: expected type `rustc_target::abi::Variants`
[00:07:40]                 found type `ty::layout::Variants`
[00:07:40] error[E0308]: mismatched types
[00:07:40]     --> librustc/ty/layout.rs:1023:41
[00:07:40]      |
