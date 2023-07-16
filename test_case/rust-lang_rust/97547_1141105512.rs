plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:337:17
    |
307 |  / macro_rules! nonzero_unsigned_operations {
308 |  |     ( $( $Ty: ident($Int: ident); )+ ) => {
309 |  |         $(
310 |  |             impl $Ty {
...    |
337 | /|                 pub const fn checked_add(self, other: $Int) -> Option<$Ty> {
338 | ||                     if let Some(result) = self.get().checked_add(other) {
339 | ||                         // SAFETY: $Int::checked_add returns None on overflow
340 | ||                         // so the result cannot be zero.
344 | ||                     }
345 | ||                 }
    | ||_________________^
...    |
...    |
497 |  |     }
498 |  | }
    |  |_- in this expansion of `nonzero_unsigned_operations!`
499 | 
500 | /  nonzero_unsigned_operations! {
501 | |      NonZeroU8(u8);
502 | |      NonZeroU16(u16);
503 | |      NonZeroU32(u32);
506 | |      NonZeroUsize(usize);
507 | |  }
    | |__- in this macro invocation


error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:371:17
    |
307 |  / macro_rules! nonzero_unsigned_operations {
308 |  |     ( $( $Ty: ident($Int: ident); )+ ) => {
309 |  |         $(
310 |  |             impl $Ty {
...    |
371 | /|                 pub const fn saturating_add(self, other: $Int) -> $Ty {
372 | ||                     // SAFETY: $Int::saturating_add returns $Int::MAX on overflow
373 | ||                     // so the result cannot be zero.
374 | ||                     unsafe { $Ty::new_unchecked(self.get().saturating_add(other)) }
    | ||_________________^
...    |
497 |  |     }
498 |  | }
498 |  | }
    |  |_- in this expansion of `nonzero_unsigned_operations!`
499 | 
500 | /  nonzero_unsigned_operations! {
501 | |      NonZeroU8(u8);
502 | |      NonZeroU16(u16);
503 | |      NonZeroU32(u32);
506 | |      NonZeroUsize(usize);
507 | |  }
    | |__- in this macro invocation


error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:436:17
    |
307 |  / macro_rules! nonzero_unsigned_operations {
308 |  |     ( $( $Ty: ident($Int: ident); )+ ) => {
309 |  |         $(
310 |  |             impl $Ty {
...    |
436 | /|                 pub const fn checked_next_power_of_two(self) -> Option<$Ty> {
437 | ||                     if let Some(nz) = self.get().checked_next_power_of_two() {
438 | ||                         // SAFETY: The next power of two is positive
439 | ||                         // and overflow is checked.
443 | ||                     }
444 | ||                 }
    | ||_________________^
...    |
...    |
497 |  |     }
498 |  | }
    |  |_- in this expansion of `nonzero_unsigned_operations!`
499 | 
500 | /  nonzero_unsigned_operations! {
501 | |      NonZeroU8(u8);
502 | |      NonZeroU16(u16);
503 | |      NonZeroU32(u32);
506 | |      NonZeroUsize(usize);
507 | |  }
    | |__- in this macro invocation


error: implementation has missing stability attribute
   --> library/core/src/num/nonzero.rs:513:13
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl Neg for $Ty {
514 | ||                 type Output = Self;
515 | ||                 /// Negate the non-zero value.
516 | ||                 ///
...   ||
---
741 |  |     }
742 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
743 | 
744 | /  nonzero_signed_operations! {
745 | |      NonZeroI8(i8) -> NonZeroU8(u8);
746 | |      NonZeroI16(i16) -> NonZeroU16(u16);
747 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
750 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
751 | |  }

error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:562:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl Neg for $Ty {
...    |
562 | /|                 pub const fn abs(self) -> $Ty {
563 | ||                     // SAFETY: This cannot overflow to zero.
564 | ||                     unsafe { $Ty::new_unchecked(self.get().abs()) }
    | ||_________________^
...    |
741 |  |     }
742 |  | }
742 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
743 | 
744 | /  nonzero_signed_operations! {
745 | |      NonZeroI8(i8) -> NonZeroU8(u8);
746 | |      NonZeroI16(i16) -> NonZeroU16(u16);
747 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
750 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
751 | |  }

error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:593:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl Neg for $Ty {
...    |
593 | /|                 pub const fn checked_abs(self) -> Option<$Ty> {
594 | ||                     if let Some(nz) = self.get().checked_abs() {
595 | ||                         // SAFETY: absolute value of nonzero cannot yield zero values.
596 | ||                         Some(unsafe { $Ty::new_unchecked(nz) })
599 | ||                     }
600 | ||                 }
    | ||_________________^
...    |
...    |
741 |  |     }
742 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
743 | 
744 | /  nonzero_signed_operations! {
745 | |      NonZeroI8(i8) -> NonZeroU8(u8);
746 | |      NonZeroI16(i16) -> NonZeroU16(u16);
747 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
750 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
751 | |  }

error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:628:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl Neg for $Ty {
...    |
628 | /|                 pub const fn overflowing_abs(self) -> ($Ty, bool) {
629 | ||                     let (nz, flag) = self.get().overflowing_abs();
630 | ||                     (
631 | ||                         // SAFETY: absolute value of nonzero cannot yield zero values.
634 | ||                     )
635 | ||                 }
    | ||_________________^
...    |
...    |
741 |  |     }
742 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
743 | 
744 | /  nonzero_signed_operations! {
745 | |      NonZeroI8(i8) -> NonZeroU8(u8);
746 | |      NonZeroI16(i16) -> NonZeroU16(u16);
747 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
750 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
751 | |  }

error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:667:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl Neg for $Ty {
...    |
667 | /|                 pub const fn saturating_abs(self) -> $Ty {
668 | ||                     // SAFETY: absolute value of nonzero cannot yield zero values.
669 | ||                     unsafe { $Ty::new_unchecked(self.get().saturating_abs()) }
    | ||_________________^
...    |
741 |  |     }
742 |  | }
742 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
743 | 
744 | /  nonzero_signed_operations! {
745 | |      NonZeroI8(i8) -> NonZeroU8(u8);
746 | |      NonZeroI16(i16) -> NonZeroU16(u16);
747 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
750 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
751 | |  }

error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:701:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl Neg for $Ty {
...    |
701 | /|                 pub const fn wrapping_abs(self) -> $Ty {
702 | ||                     // SAFETY: absolute value of nonzero cannot yield zero values.
703 | ||                     unsafe { $Ty::new_unchecked(self.get().wrapping_abs()) }
    | ||_________________^
...    |
741 |  |     }
742 |  | }
742 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
743 | 
744 | /  nonzero_signed_operations! {
745 | |      NonZeroI8(i8) -> NonZeroU8(u8);
746 | |      NonZeroI16(i16) -> NonZeroU16(u16);
747 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
750 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
751 | |  }

error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:735:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl Neg for $Ty {
...    |
735 | /|                 pub const fn unsigned_abs(self) -> $Uty {
736 | ||                     // SAFETY: absolute value of nonzero cannot yield zero values.
737 | ||                     unsafe { $Uty::new_unchecked(self.get().unsigned_abs()) }
    | ||_________________^
...    |
741 |  |     }
742 |  | }
742 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
743 | 
744 | /  nonzero_signed_operations! {
745 | |      NonZeroI8(i8) -> NonZeroU8(u8);
746 | |      NonZeroI16(i16) -> NonZeroU16(u16);
747 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
750 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
751 | |  }

error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:783:17
    |
    |
754 |  / macro_rules! nonzero_unsigned_signed_operations {
755 |  |     ( $( $signedness:ident $Ty: ident($Int: ty); )+ ) => {
756 |  |         $(
757 |  |             impl $Ty {
...    |
783 | /|                 pub const fn checked_mul(self, other: $Ty) -> Option<$Ty> {
784 | ||                     if let Some(result) = self.get().checked_mul(other.get()) {
785 | ||                         // SAFETY: checked_mul returns None on overflow
786 | ||                         // and `other` is also non-null
791 | ||                     }
792 | ||                 }
    | ||_________________^
...    |
...    |
940 |  |     }
941 |  | }
    |  |_- in this expansion of `nonzero_unsigned_signed_operations!`
...
953 | /  nonzero_unsigned_signed_operations! {
954 | |      unsigned NonZeroU8(u8);
955 | |      unsigned NonZeroU16(u16);
956 | |      unsigned NonZeroU32(u32);
965 | |      signed NonZeroIsize(isize);
966 | |  }
    | |__- in this macro invocation


error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:818:17
    |
754 |  / macro_rules! nonzero_unsigned_signed_operations {
755 |  |     ( $( $signedness:ident $Ty: ident($Int: ty); )+ ) => {
756 |  |         $(
757 |  |             impl $Ty {
...    |
818 | /|                 pub const fn saturating_mul(self, other: $Ty) -> $Ty {
819 | ||                     // SAFETY: saturating_mul returns u*::MAX on overflow
820 | ||                     // and `other` is also non-null
821 | ||                     // so the result cannot be zero.
822 | ||                     unsafe { $Ty::new_unchecked(self.get().saturating_mul(other.get())) }
    | ||_________________^
...    |
940 |  |     }
941 |  | }
941 |  | }
    |  |_- in this expansion of `nonzero_unsigned_signed_operations!`
...
953 | /  nonzero_unsigned_signed_operations! {
954 | |      unsigned NonZeroU8(u8);
955 | |      unsigned NonZeroU16(u16);
956 | |      unsigned NonZeroU32(u32);
965 | |      signed NonZeroIsize(isize);
966 | |  }
    | |__- in this macro invocation


error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:890:17
    |
754 |  / macro_rules! nonzero_unsigned_signed_operations {
755 |  |     ( $( $signedness:ident $Ty: ident($Int: ty); )+ ) => {
756 |  |         $(
757 |  |             impl $Ty {
...    |
890 | /|                 pub const fn checked_pow(self, other: u32) -> Option<$Ty> {
891 | ||                     if let Some(result) = self.get().checked_pow(other) {
892 | ||                         // SAFETY: checked_pow returns None on overflow
893 | ||                         // so the result cannot be zero.
897 | ||                     }
898 | ||                 }
    | ||_________________^
...    |
...    |
940 |  |     }
941 |  | }
    |  |_- in this expansion of `nonzero_unsigned_signed_operations!`
...
953 | /  nonzero_unsigned_signed_operations! {
954 | |      unsigned NonZeroU8(u8);
955 | |      unsigned NonZeroU16(u16);
956 | |      unsigned NonZeroU32(u32);
965 | |      signed NonZeroIsize(isize);
966 | |  }
    | |__- in this macro invocation


error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:933:17
    |
754 |  / macro_rules! nonzero_unsigned_signed_operations {
755 |  |     ( $( $signedness:ident $Ty: ident($Int: ty); )+ ) => {
756 |  |         $(
757 |  |             impl $Ty {
...    |
933 | /|                 pub const fn saturating_pow(self, other: u32) -> $Ty {
934 | ||                     // SAFETY: saturating_pow returns u*::MAX on overflow
935 | ||                     // so the result cannot be zero.
936 | ||                     unsafe { $Ty::new_unchecked(self.get().saturating_pow(other)) }
    | ||_________________^
...    |
940 |  |     }
941 |  | }
941 |  | }
    |  |_- in this expansion of `nonzero_unsigned_signed_operations!`
...
953 | /  nonzero_unsigned_signed_operations! {
954 | |      unsigned NonZeroU8(u8);
955 | |      unsigned NonZeroU16(u16);
956 | |      unsigned NonZeroU32(u32);
965 | |      signed NonZeroIsize(isize);
966 | |  }
    | |__- in this macro invocation

