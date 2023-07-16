plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `checked_add`
   --> library/core/src/num/int_macros.rs:409:9
    |
409 | /         /// Unchecked integer addition. Computes `self + rhs`, assuming overflow
410 | |         /// cannot occur.
411 | |         ///
412 | |         /// # Safety
...   |
416 | |         /// i.e. when [`checked_add`] would return `None`.
417 | |         #[doc = concat!("[`checked_add`]: ", stringify!($SelfT), "::checked_add")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            i.e. when [`checked_add`] would return `None`.
    = note: no item named `checked_add` in scope
    = note: no item named `checked_add` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_add`
   --> library/core/src/num/int_macros.rs:409:9
    |
    |
409 | /         /// Unchecked integer addition. Computes `self + rhs`, assuming overflow
410 | |         /// cannot occur.
411 | |         ///
412 | |         /// # Safety
...   |
416 | |         /// i.e. when [`checked_add`] would return `None`.
417 | |         #[doc = concat!("[`checked_add`]: ", stringify!($SelfT), "::checked_add")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_add`]: i8::checked_add
    = note: no item named `checked_add` in scope
    = note: no item named `checked_add` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_sub`
   --> library/core/src/num/int_macros.rs:454:9
    |
    |
454 | /         /// Unchecked integer subtraction. Computes `self - rhs`, assuming overflow
455 | |         /// cannot occur.
456 | |         ///
457 | |         /// # Safety
...   |
461 | |         /// i.e. when [`checked_sub`] would return `None`.
462 | |         #[doc = concat!("[`checked_sub`]: ", stringify!($SelfT), "::checked_sub")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_sub`] would return `None`.
    = note: no item named `checked_sub` in scope
    = note: no item named `checked_sub` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_sub`
   --> library/core/src/num/int_macros.rs:454:9
    |
    |
454 | /         /// Unchecked integer subtraction. Computes `self - rhs`, assuming overflow
455 | |         /// cannot occur.
456 | |         ///
457 | |         /// # Safety
...   |
461 | |         /// i.e. when [`checked_sub`] would return `None`.
462 | |         #[doc = concat!("[`checked_sub`]: ", stringify!($SelfT), "::checked_sub")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_sub`]: i8::checked_sub
    = note: no item named `checked_sub` in scope
    = note: no item named `checked_sub` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_mul`
   --> library/core/src/num/int_macros.rs:499:9
    |
    |
499 | /         /// Unchecked integer multiplication. Computes `self * rhs`, assuming overflow
500 | |         /// cannot occur.
501 | |         ///
502 | |         /// # Safety
...   |
506 | |         /// i.e. when [`checked_mul`] would return `None`.
507 | |         #[doc = concat!("[`checked_mul`]: ", stringify!($SelfT), "::checked_mul")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_mul`] would return `None`.
    = note: no item named `checked_mul` in scope
    = note: no item named `checked_mul` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_mul`
   --> library/core/src/num/int_macros.rs:499:9
    |
    |
499 | /         /// Unchecked integer multiplication. Computes `self * rhs`, assuming overflow
500 | |         /// cannot occur.
501 | |         ///
502 | |         /// # Safety
...   |
506 | |         /// i.e. when [`checked_mul`] would return `None`.
507 | |         #[doc = concat!("[`checked_mul`]: ", stringify!($SelfT), "::checked_mul")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_mul`]: i8::checked_mul
    = note: no item named `checked_mul` in scope
    = note: no item named `checked_mul` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_shl`
   --> library/core/src/num/int_macros.rs:666:9
    |
    |
666 | /         /// Unchecked shift left. Computes `self << rhs`, assuming that
667 | |         /// `rhs` is less than the number of bits in `self`.
668 | |         ///
669 | |         /// # Safety
...   |
673 | |         /// i.e. when [`checked_shl`] would return `None`.
674 | |         #[doc = concat!("[`checked_shl`]: ", stringify!($SelfT), "::checked_shl")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_shl`] would return `None`.
    = note: no item named `checked_shl` in scope
    = note: no item named `checked_shl` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_shl`
   --> library/core/src/num/int_macros.rs:666:9
    |
    |
666 | /         /// Unchecked shift left. Computes `self << rhs`, assuming that
667 | |         /// `rhs` is less than the number of bits in `self`.
668 | |         ///
669 | |         /// # Safety
...   |
673 | |         /// i.e. when [`checked_shl`] would return `None`.
674 | |         #[doc = concat!("[`checked_shl`]: ", stringify!($SelfT), "::checked_shl")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_shl`]: i8::checked_shl
    = note: no item named `checked_shl` in scope
    = note: no item named `checked_shl` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_shr`
   --> library/core/src/num/int_macros.rs:711:9
    |
    |
711 | /         /// Unchecked shift right. Computes `self >> rhs`, assuming that
712 | |         /// `rhs` is less than the number of bits in `self`.
713 | |         ///
714 | |         /// # Safety
...   |
718 | |         /// i.e. when [`checked_shr`] would return `None`.
719 | |         #[doc = concat!("[`checked_shr`]: ", stringify!($SelfT), "::checked_shr")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_shr`] would return `None`.
    = note: no item named `checked_shr` in scope
    = note: no item named `checked_shr` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_shr`
   --> library/core/src/num/int_macros.rs:711:9
    |
    |
711 | /         /// Unchecked shift right. Computes `self >> rhs`, assuming that
712 | |         /// `rhs` is less than the number of bits in `self`.
713 | |         ///
714 | |         /// # Safety
...   |
718 | |         /// i.e. when [`checked_shr`] would return `None`.
719 | |         #[doc = concat!("[`checked_shr`]: ", stringify!($SelfT), "::checked_shr")]
    | 
   ::: library/core/src/num/mod.rs:92:5
    |
    |
92  | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
93  | |     "[0x12]", "[0x12]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_shr`]: i8::checked_shr
    = note: no item named `checked_shr` in scope
    = note: no item named `checked_shr` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_add`
   --> library/core/src/num/int_macros.rs:409:9
    |
    |
409 | /         /// Unchecked integer addition. Computes `self + rhs`, assuming overflow
410 | |         /// cannot occur.
411 | |         ///
412 | |         /// # Safety
...   |
416 | |         /// i.e. when [`checked_add`] would return `None`.
417 | |         #[doc = concat!("[`checked_add`]: ", stringify!($SelfT), "::checked_add")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_add`] would return `None`.
    = note: no item named `checked_add` in scope
    = note: no item named `checked_add` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_add`
   --> library/core/src/num/int_macros.rs:409:9
    |
    |
409 | /         /// Unchecked integer addition. Computes `self + rhs`, assuming overflow
410 | |         /// cannot occur.
411 | |         ///
412 | |         /// # Safety
...   |
416 | |         /// i.e. when [`checked_add`] would return `None`.
417 | |         #[doc = concat!("[`checked_add`]: ", stringify!($SelfT), "::checked_add")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_add`]: i16::checked_add
    = note: no item named `checked_add` in scope
    = note: no item named `checked_add` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_sub`
   --> library/core/src/num/int_macros.rs:454:9
    |
    |
454 | /         /// Unchecked integer subtraction. Computes `self - rhs`, assuming overflow
455 | |         /// cannot occur.
456 | |         ///
457 | |         /// # Safety
...   |
461 | |         /// i.e. when [`checked_sub`] would return `None`.
462 | |         #[doc = concat!("[`checked_sub`]: ", stringify!($SelfT), "::checked_sub")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_sub`] would return `None`.
    = note: no item named `checked_sub` in scope
    = note: no item named `checked_sub` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_sub`
   --> library/core/src/num/int_macros.rs:454:9
    |
    |
454 | /         /// Unchecked integer subtraction. Computes `self - rhs`, assuming overflow
455 | |         /// cannot occur.
456 | |         ///
457 | |         /// # Safety
...   |
461 | |         /// i.e. when [`checked_sub`] would return `None`.
462 | |         #[doc = concat!("[`checked_sub`]: ", stringify!($SelfT), "::checked_sub")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_sub`]: i16::checked_sub
    = note: no item named `checked_sub` in scope
    = note: no item named `checked_sub` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_mul`
   --> library/core/src/num/int_macros.rs:499:9
    |
    |
499 | /         /// Unchecked integer multiplication. Computes `self * rhs`, assuming overflow
500 | |         /// cannot occur.
501 | |         ///
502 | |         /// # Safety
...   |
506 | |         /// i.e. when [`checked_mul`] would return `None`.
507 | |         #[doc = concat!("[`checked_mul`]: ", stringify!($SelfT), "::checked_mul")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_mul`] would return `None`.
    = note: no item named `checked_mul` in scope
    = note: no item named `checked_mul` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_mul`
   --> library/core/src/num/int_macros.rs:499:9
    |
    |
499 | /         /// Unchecked integer multiplication. Computes `self * rhs`, assuming overflow
500 | |         /// cannot occur.
501 | |         ///
502 | |         /// # Safety
...   |
506 | |         /// i.e. when [`checked_mul`] would return `None`.
507 | |         #[doc = concat!("[`checked_mul`]: ", stringify!($SelfT), "::checked_mul")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_mul`]: i16::checked_mul
    = note: no item named `checked_mul` in scope
    = note: no item named `checked_mul` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_shl`
   --> library/core/src/num/int_macros.rs:666:9
    |
    |
666 | /         /// Unchecked shift left. Computes `self << rhs`, assuming that
667 | |         /// `rhs` is less than the number of bits in `self`.
668 | |         ///
669 | |         /// # Safety
...   |
673 | |         /// i.e. when [`checked_shl`] would return `None`.
674 | |         #[doc = concat!("[`checked_shl`]: ", stringify!($SelfT), "::checked_shl")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_shl`] would return `None`.
    = note: no item named `checked_shl` in scope
    = note: no item named `checked_shl` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_shl`
   --> library/core/src/num/int_macros.rs:666:9
    |
    |
666 | /         /// Unchecked shift left. Computes `self << rhs`, assuming that
667 | |         /// `rhs` is less than the number of bits in `self`.
668 | |         ///
669 | |         /// # Safety
...   |
673 | |         /// i.e. when [`checked_shl`] would return `None`.
674 | |         #[doc = concat!("[`checked_shl`]: ", stringify!($SelfT), "::checked_shl")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            [`checked_shl`]: i16::checked_shl
    = note: no item named `checked_shl` in scope
    = note: no item named `checked_shl` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_shr`
   --> library/core/src/num/int_macros.rs:711:9
    |
    |
711 | /         /// Unchecked shift right. Computes `self >> rhs`, assuming that
712 | |         /// `rhs` is less than the number of bits in `self`.
713 | |         ///
714 | |         /// # Safety
...   |
718 | |         /// i.e. when [`checked_shr`] would return `None`.
719 | |         #[doc = concat!("[`checked_shr`]: ", stringify!($SelfT), "::checked_shr")]
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
    |
98  | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
99  | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    |
    = note: the link appears in this line:
            
            
            i.e. when [`checked_shr`] would return `None`.
    = note: no item named `checked_shr` in scope
    = note: no item named `checked_shr` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `checked_shr`
   --> library/core/src/num/int_macros.rs:711:9
    |
    |
711 | /         /// Unchecked shift right. Computes `self >> rhs`, assuming that
