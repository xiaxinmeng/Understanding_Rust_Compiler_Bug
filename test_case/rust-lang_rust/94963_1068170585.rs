plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `isize_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1655:44
     |
1655 |                 Isize => single(lang_items.isize_impl()),
     |                                            ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `i8_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1656:41
     |
1656 |                 I8 => single(lang_items.i8_impl()),
     |                                         ^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `i16_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1657:42
     |
1657 |                 I16 => single(lang_items.i16_impl()),
     |                                          ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `i32_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1658:42
     |
1658 |                 I32 => single(lang_items.i32_impl()),
     |                                          ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `i64_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1659:42
     |
1659 |                 I64 => single(lang_items.i64_impl()),
     |                                          ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `i128_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1660:43
     |
1660 |                 I128 => single(lang_items.i128_impl()),
     |                                           ^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `usize_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1661:44
     |
1661 |                 Usize => single(lang_items.usize_impl()),
     |                                            ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `u8_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1662:41
     |
1662 |                 U8 => single(lang_items.u8_impl()),
     |                                         ^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `u16_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1663:42
     |
1663 |                 U16 => single(lang_items.u16_impl()),
     |                                          ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `u32_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1664:42
     |
1664 |                 U32 => single(lang_items.u32_impl()),
     |                                          ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `u64_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1665:42
     |
1665 |                 U64 => single(lang_items.u64_impl()),
     |                                          ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `u128_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1666:43
     |
1666 |                 U128 => single(lang_items.u128_impl()),
     |                                           ^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `f32_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1667:40
     |
1667 |                 F32 => both(lang_items.f32_impl(), lang_items.f32_runtime_impl()),
     |                                        ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `f32_runtime_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1667:63
     |
1667 |                 F32 => both(lang_items.f32_impl(), lang_items.f32_runtime_impl()),
     |                                                               ^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `f64_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1668:40
     |
1668 |                 F64 => both(lang_items.f64_impl(), lang_items.f64_runtime_impl()),
     |                                        ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `f64_runtime_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1668:63
     |
1668 |                 F64 => both(lang_items.f64_impl(), lang_items.f64_runtime_impl()),
     |                                                               ^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `char_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1669:43
     |
1669 |                 Char => single(lang_items.char_impl()),
     |                                           ^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `bool_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1670:43
     |
1670 |                 Bool => single(lang_items.bool_impl()),
     |                                           ^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `str_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1671:40
     |
1671 |                 Str => both(lang_items.str_impl(), lang_items.str_alloc_impl()),
     |                                        ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `str_alloc_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1671:63
     |
1671 |                 Str => both(lang_items.str_impl(), lang_items.str_alloc_impl()),
     |                                                               ^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1674:26
1674 |                         .slice_impl()
1674 |                         .slice_impl()
     |                          ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_u8_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1676:43
     |
1676 |                         .chain(lang_items.slice_u8_impl())
     |                                           ^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_alloc_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1677:43
     |
1677 |                         .chain(lang_items.slice_alloc_impl())
     |                                           ^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_u8_alloc_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1678:43
     |
1678 |                         .chain(lang_items.slice_u8_alloc_impl())
     |                                           ^^^^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `array_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1681:44
     |
1681 |                 Array => single(lang_items.array_impl()),
     |                                            ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `const_ptr_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1686:26
1686 |                         .const_ptr_impl()
1686 |                         .const_ptr_impl()
     |                          ^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `mut_ptr_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1688:43
     |
1688 |                         .chain(lang_items.mut_ptr_impl())
     |                                           ^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `const_slice_ptr_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1689:43
     |
1689 |                         .chain(lang_items.const_slice_ptr_impl())
     |                                           ^^^^^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `mut_slice_ptr_impl` found for reference `&LanguageItems` in the current scope
    --> src/librustdoc/clean/types.rs:1690:43
     |
1690 |                         .chain(lang_items.mut_slice_ptr_impl())
     |                                           ^^^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to 29 previous errors
Build completed unsuccessfully in 0:02:43
