plain
[00:05:38]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:46]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:21]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:38]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:49] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::Variants` in the current scope
[00:07:49]     --> librustc/macros.rs:104:27
[00:07:49]      |
[00:07:49] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:07:49]      | 
[00:07:49]      | 
[00:07:49]     ::: librustc/ty/layout.rs:2002:1
[00:07:49]      |
[00:07:49] 2002 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:07:49] 2003 | |     variants,
[00:07:49] 2004 | |     fields,
[00:07:49] 2005 | |     abi,
[00:07:49] 2007 | |     align
[00:07:49] 2008 | | });
[00:07:49] 2008 | | });
[00:07:49]      | |___- in this macro invocation
[00:07:49]      |
[00:07:49]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:07:49]              `&rustc_target::abi::Variants : rustc_data_structures::stable_hasher::HashStable<_>`
[00:07:49] 
[00:07:49] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::FieldPlacement` in the current scope
[00:07:49]     --> librustc/macros.rs:104:27
[00:07:49]      |
[00:07:49] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:07:49]      | 
[00:07:49]      | 
[00:07:49]     ::: librustc/ty/layout.rs:2002:1
[00:07:49]      |
[00:07:49] 2002 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:07:49] 2003 | |     variants,
[00:07:49] 2004 | |     fields,
[00:07:49] 2005 | |     abi,
[00:07:49] 2007 | |     align
[00:07:49] 2008 | | });
[00:07:49] 2008 | | });
[00:07:49]      | |___- in this macro invocation
[00:07:49]      |
[00:07:49]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:07:49]              `&rustc_target::abi::FieldPlacement : rustc_data_structures::stable_hasher::HashStable<_>`
[00:07:49] 
[00:07:49] error[E0599]: no method named `hash_stable` found for type `&rustc_target::abi::Abi` in the current scope
[00:07:49]     --> librustc/macros.rs:104:27
[00:07:49]      |
[00:07:49] 104  |                   $( $field.hash_stable(__ctx, __hasher));*
[00:07:49]      | 
[00:07:49]      | 
[00:07:49]     ::: librustc/ty/layout.rs:2002:1
[00:07:49]      |
[00:07:49] 2002 | / impl_stable_hash_for!(struct ::ty::layout::LayoutDetails {
[00:07:49] 2003 | |     variants,
[00:07:49] 2004 | |     fields,
[00:07:49] 2005 | |     abi,
[00:07:49] 2007 | |     align
[00:07:49] 2008 | | });
[00:07:49] 2008 | | });
[00:07:49]      | |___- in this macro invocation
[00:07:49]      |
[00:07:49]      = note: the method `hash_stable` exists but the following trait bounds were not satisfied:
[00:07:49]              `&rustc_target::abi::Abi : rustc_data_structures::stable_hasher::HashStable<_>`
[00:07:49] 
[00:08:02] error[E0277]: the trait bound `ty::layout::LayoutError<'_>: serialize::UseSpecializedEncodable` is not satisfied
[00:08:02]   --> librustc/mir/interpret/error.rs:33:17
[00:08:02]    |
[00:08:02] 33 | #[derive(Clone, RustcEncodable, RustcDecodable)]
[00:08:02]    |                 ^^^^^^^^^^^^^^ the trait `serialize::UseSpecializedEncodable` is not implemented for `ty::layout::LayoutError<'_>`
[00:08:02]    |
[00:08:02]    = note: required because of the requirements on the impl of `serialize::Encodable` for `ty::layout::LayoutError<'_>`
[00:08:02]    = note: required by `serialize::Encodable::encode`
[00:08:02] 
[00:08:02] error[E0277]: the trait bound `ty::layout::LayoutError<'_>: serialize::UseSpecializedDecodable` is not satisfied
[00:08:02]   --> librustc/mir/interpret/error.rs:33:33
[00:08:03]    |
[00:08:03] 33 | #[derive(Clone, RustcEncodable, RustcDecodable)]
[00:08:03]    |                                 ^^^^^^^^^^^^^^ the trait `serialize::UseSpecializedDecodable` is not implemented for `ty::layout::LayoutError<'_>`
[00:08:03]    |
[00:08:03]    = note: required because of the requirements on the impl of `serialize::Decodable` for `ty::layout::LayoutError<'_>`
[00:08:03]    = note: required by `serialize::Decodable::decode`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:409:27
[00:08:08]     |
[00:08:08]     |
[00:08:08] 409 |                 variants: Variants::Single { index: 0 },
[00:08:08]     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:08]     = note: expected type `rustc_target::abi::Variants`
[00:08:08]     = note: expected type `rustc_target::abi::Variants`
[00:08:08]                found type `ty::layout::Variants`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:410:25
[00:08:08]     |
[00:08:08]     |
[00:08:08] 410 |                   fields: FieldPlacement::Arbitrary {
[00:08:08]     |  _________________________^
[00:08:08] 411 | |                     offsets: vec![Size::from_bytes(0), b_offset],
[00:08:08] 412 | |                     memory_index: vec![0, 1]
[00:08:08] 413 | |                 },
[00:08:08]     | |_________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:08]     |
[00:08:08]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:08]                found type `ty::layout::FieldPlacement`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:414:22
[00:08:08]     |
[00:08:08]     |
[00:08:08] 414 |                 abi: Abi::ScalarPair(a, b),
[00:08:08]     |                      ^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]                found type `ty::layout::Abi`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:506:33
[00:08:08]     |
[00:08:08]     |
[00:08:08] 506 |                 if field.abi == Abi::Uninhabited {
[00:08:08]     |                                 ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]                found type `ty::layout::Abi`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:596:60
[00:08:08]     |
[00:08:08]     |
[00:08:08] 596 |                             details: &LayoutDetails { abi: Abi::Scalar(ref a), .. }, ..
[00:08:08]     |                                                            ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]                found type `ty::layout::Abi`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:598:60
[00:08:08]     |
[00:08:08]     |
[00:08:08] 598 |                             details: &LayoutDetails { abi: Abi::Scalar(ref b), .. }, ..
[00:08:08]     |                                                            ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]                found type `ty::layout::Abi`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:581:37
[00:08:08]     |
[00:08:08]     |
[00:08:08] 581 |                                     Abi::Scalar(_) | Abi::Vector { .. } if optimize => {
[00:08:08]     |                                     ^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]                found type `ty::layout::Abi`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:581:54
[00:08:08]     |
[00:08:08]     |
[00:08:08] 581 |                                     Abi::Scalar(_) | Abi::Vector { .. } if optimize => {
[00:08:08]     |                                                      ^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]                found type `ty::layout::Abi`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:586:37
[00:08:08]     |
[00:08:08]     |
[00:08:08] 586 |                                     Abi::ScalarPair(..) => {
[00:08:08]     |                                     ^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]     = note: expected type `rustc_target::abi::Abi`
[00:08:08]                found type `ty::layout::Abi`
[00:08:08] error[E0308]: mismatched types
[00:08:08]    --> librustc/ty/layout.rs:582:47
[00:08:08]     |
[00:08:08]     |
[00:08:08] 582 |                                         abi = field.abi.clone();
[00:08:08]     |                                               ^^^^^^^^^^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:08:08]     |
[00:08:08]     = note: expected type `ty::layout::Abi`
[00:08:08] 
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:587:47
[00:08:09]     |
[00:08:09]     |
[00:08:09] 587 |                                         abi = field.abi.clone();
[00:08:09]     |                                               ^^^^^^^^^^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:08:09]     |
[00:08:09]     = note: expected type `ty::layout::Abi`
[00:08:09] 
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:608:33
[00:08:09]     |
[00:08:09]     |
[00:08:09] 608 | /                                 FieldPlacement::Arbitrary {
[00:08:09] 609 | |                                     ref offsets,
[00:08:09] 610 | |                                     ref memory_index
[00:08:09] 611 | |                                 } => {
[00:08:09]     | |_________________________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:09]     |
[00:08:09]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:09]                found type `ty::layout::FieldPlacement`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:623:39
[00:08:09]     |
[00:08:09]     |
[00:08:09] 623 |                                 abi = pair.abi;
[00:08:09]     |                                       ^^^^^^^^ expected enum `ty::layout::Abi`, found enum `rustc_target::abi::Abi`
[00:08:09]     |
[00:08:09]     = note: expected type `ty::layout::Abi`
[00:08:09] 
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:633:27
[00:08:09]     |
[00:08:09]     |
[00:08:09] 633 |                 variants: Variants::Single { index: 0 },
[00:08:09]     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:09]     = note: expected type `rustc_target::abi::Variants`
[00:08:09]     = note: expected type `rustc_target::abi::Variants`
[00:08:09]                found type `ty::layout::Variants`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:634:25
[00:08:09]     |
[00:08:09]     |
[00:08:09] 634 |                   fields: FieldPlacement::Arbitrary {
[00:08:09] 635 | |                     offsets,
[00:08:09] 636 | |                     memory_index
[00:08:09] 637 | |                 },
[00:08:09] 637 | |                 },
[00:08:09]     | |_________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:09]     |
[00:08:09]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:09]                found type `ty::layout::FieldPlacement`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:638:17
[00:08:09]     |
[00:08:09] 638 |                 abi,
[00:08:09] 638 |                 abi,
[00:08:09]     |                 ^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:09]     = note: expected type `rustc_target::abi::Abi`
[00:08:09]     = note: expected type `rustc_target::abi::Abi`
[00:08:09]                found type `ty::layout::Abi`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:729:31
[00:08:09]     |
[00:08:09]     |
[00:08:09] 729 |                     variants: Variants::Single { index: 0 },
[00:08:09]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:09]     = note: expected type `rustc_target::abi::Variants`
[00:08:09]     = note: expected type `rustc_target::abi::Variants`
[00:08:09]                found type `ty::layout::Variants`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:730:29
[00:08:09]     |
[00:08:09]     |
[00:08:09] 730 |                       fields: FieldPlacement::Array {
[00:08:09]     |  _____________________________^
[00:08:09] 731 | |                         stride: element.size,
[00:08:09] 733 | |                     },
[00:08:09] 733 | |                     },
[00:08:09]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:09]     |
[00:08:09]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:09]                found type `ty::layout::FieldPlacement`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:734:26
[00:08:09]     |
[00:08:09]     |
[00:08:09] 734 |                     abi: Abi::Aggregate { sized: true },
[00:08:09]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:09]     = note: expected type `rustc_target::abi::Abi`
[00:08:09]     = note: expected type `rustc_target::abi::Abi`
[00:08:09]                found type `ty::layout::Abi`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:742:31
[00:08:09]     |
[00:08:09]     |
[00:08:09] 742 |                     variants: Variants::Single { index: 0 },
[00:08:09]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:09]     = note: expected type `rustc_target::abi::Variants`
[00:08:09]     = note: expected type `rustc_target::abi::Variants`
[00:08:09]                found type `ty::layout::Variants`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:743:29
[00:08:09]     |
[00:08:09]     |
[00:08:09] 743 |                       fields: FieldPlacement::Array {
[00:08:09]     |  _____________________________^
[00:08:09] 744 | |                         stride: element.size,
[00:08:09] 746 | |                     },
[00:08:09] 746 | |                     },
[00:08:09]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:09]     |
[00:08:09]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:09]                found type `ty::layout::FieldPlacement`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:747:26
[00:08:09]     |
[00:08:09]     |
[00:08:09] 747 |                     abi: Abi::Aggregate { sized: false },
[00:08:09]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:09]     = note: expected type `rustc_target::abi::Abi`
[00:08:09]     = note: expected type `rustc_target::abi::Abi`
[00:08:09]                found type `ty::layout::Abi`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:754:31
[00:08:09]     |
[00:08:09]     |
[00:08:09] 754 |                     variants: Variants::Single { index: 0 },
[00:08:09]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:09]     = note: expected type `rustc_target::abi::Variants`
[00:08:09]     = note: expected type `rustc_target::abi::Variants`
[00:08:09]                found type `ty::layout::Variants`
[00:08:09] error[E0308]: mismatched types
[00:08:09]    --> librustc/ty/layout.rs:755:29
[00:08:09]     |
[00:08:09]     |
[00:08:09] 755 |                       fields: FieldPlacement::Array {
[00:08:09]     |  _____________________________^
[00:08:09] 756 | |                         stride: Size::from_bytes(1),
[00:08:09] 758 | |                     },
[00:08:09] 758 | |                     },
[00:08:09]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:09]     |
[00:08:09]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:09]                found type `ty::layout::FieldPlacement`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:759:26
[00:08:10]     |
[00:08:10]     |
[00:08:10] 759 |                     abi: Abi::Aggregate { sized: false },
[00:08:10]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:773:21
[00:08:10]     |
[00:08:10]     |
[00:08:10] 773 |                     Abi::Aggregate { ref mut sized } => *sized = false,
[00:08:10]     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:811:21
[00:08:10]     |
[00:08:10]     |
[00:08:10] 811 |                     Abi::Scalar(ref scalar) => scalar.clone(),
[00:08:10]     |                     ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:824:31
[00:08:10]     |
[00:08:10]     |
[00:08:10] 824 |                     variants: Variants::Single { index: 0 },
[00:08:10]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:10]     = note: expected type `rustc_target::abi::Variants`
[00:08:10]     = note: expected type `rustc_target::abi::Variants`
[00:08:10]                found type `ty::layout::Variants`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:825:29
[00:08:10]     |
[00:08:10]     |
[00:08:10] 825 |                       fields: FieldPlacement::Array {
[00:08:10]     |  _____________________________^
[00:08:10] 826 | |                         stride: element.size,
[00:08:10] 828 | |                     },
[00:08:10] 828 | |                     },
[00:08:10]     | |_____________________^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:10]     |
[00:08:10]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:10]                found type `ty::layout::FieldPlacement`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:829:26
[00:08:10]     |
[00:08:10]     |
[00:08:10] 829 |                       abi: Abi::Vector {
[00:08:10]     |  __________________________^
[00:08:10] 830 | |                         element: scalar,
[00:08:10] 832 | |                     },
[00:08:10] 832 | |                     },
[00:08:10]     | |_____________________^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:884:35
[00:08:10]     |
[00:08:10]     |
[00:08:10] 884 |                         variants: Variants::Single { index: 0 },
[00:08:10]     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:10]     = note: expected type `rustc_target::abi::Variants`
[00:08:10]     = note: expected type `rustc_target::abi::Variants`
[00:08:10]                found type `ty::layout::Variants`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:885:33
[00:08:10]     |
[00:08:10]     |
[00:08:10] 885 |                         fields: FieldPlacement::Union(variants[0].len()),
[00:08:10]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::FieldPlacement`, found enum `ty::layout::FieldPlacement`
[00:08:10]     |
[00:08:10]     = note: expected type `rustc_target::abi::FieldPlacement`
[00:08:10]                found type `ty::layout::FieldPlacement`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:886:30
[00:08:10]     |
[00:08:10]     |
[00:08:10] 886 |                         abi: Abi::Aggregate { sized: true },
[00:08:10]     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:894:61
[00:08:10]     |
[00:08:10]     |
[00:08:10] 894 |                         variants[v].iter().all(|f| f.abi != Abi::Uninhabited)
[00:08:10]     |                                                             ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:925:35
[00:08:10]     |
[00:08:10]     |
[00:08:10] 925 |                     st.variants = Variants::Single { index: v };
[00:08:10]     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:10]     = note: expected type `rustc_target::abi::Variants`
[00:08:10]     = note: expected type `rustc_target::abi::Variants`
[00:08:10]                found type `ty::layout::Variants`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:929:29
[00:08:10]     |
[00:08:10]     |
[00:08:10] 929 |                             Abi::Scalar(ref mut scalar) |
[00:08:10]     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:930:29
[00:08:10]     |
[00:08:10]     |
[00:08:10] 930 |                             Abi::ScalarPair(ref mut scalar, _) => {
[00:08:10]     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:954:59
[00:08:10]     |
[00:08:10]     |
[00:08:10] 954 |                         if fields.iter().any(|f| f.abi == Abi::Uninhabited) {
[00:08:10]     |                                                           ^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:987:47
[00:08:10]     |
[00:08:10]     |
[00:08:10] 987 |                                 st.variants = Variants::Single { index: j };
[00:08:10]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:10]     = note: expected type `rustc_target::abi::Variants`
[00:08:10]     = note: expected type `rustc_target::abi::Variants`
[00:08:10]                found type `ty::layout::Variants`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:998:33
[00:08:10]     |
[00:08:10]     |
[00:08:10] 998 |                                 Abi::Scalar(_) => Abi::Scalar(niche.clone()),
[00:08:10]     |                                 ^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]    --> librustc/ty/layout.rs:999:33
[00:08:10]     |
[00:08:10]     |
[00:08:10] 999 |                                 Abi::ScalarPair(ref first, ref second) => {
[00:08:10]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_target::abi::Abi`, found enum `ty::layout::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]     = note: expected type `rustc_target::abi::Abi`
[00:08:10]                found type `ty::layout::Abi`
[00:08:10] error[E0308]: mismatched types
[00:08:10]     --> librustc/ty/layout.rs:1015:43
[00:08:10]      |
[00:08:10]      |
[00:08:10] 1015 |                                   variants: Variants::NicheFilling {
[00:08:10]      |  ___________________________________________^
[00:08:10] 1016 | |                                     dataful_variant: i,
[00:08:10] 1017 | |                                     niche_variants,
[00:08:10] 1018 | |                                     niche,
[00:08:10] 1019 | |                                     niche_start,
[00:08:10] 1020 | |                                     variants: st,
[00:08:10] 1021 | |                                 },
[00:08:10]      | |_________________________________^ expected enum `rustc_target::abi::Variants`, found enum `ty::layout::Variants`
[00:08:10]      = note: expected type `rustc_target::abi::Variants`
[00:08:10]      = note: expected type `rustc_target::abi::Variants`
[00:08:10]                 found type `ty::layout::Variants`
[00:08:10] error[E0308]: mismatched types
[00:08:10]     --> librustc/ty/layout.rs:1022:41
[00:08:10]      |
