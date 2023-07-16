plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:333:17
    |
307 | / macro_rules! nonzero_unsigned_operations {
308 | |     ( $( $Ty: ident($Int: ident); )+ ) => {
309 | |         $(
310 | |             impl $Ty {
...   |
333 | |                 #[stable(since = "1.63")]
...   |
497 | |     }
498 | | }
    | |_- in this expansion of `nonzero_unsigned_operations!`
    | |_- in this expansion of `nonzero_unsigned_operations!`
499 | 
500 | / nonzero_unsigned_operations! {
501 | |     NonZeroU8(u8);
502 | |     NonZeroU16(u16);
503 | |     NonZeroU32(u32);
506 | |     NonZeroUsize(usize);
507 | | }
    | |_- in this macro invocation


error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:367:17
    |
307 | / macro_rules! nonzero_unsigned_operations {
308 | |     ( $( $Ty: ident($Int: ident); )+ ) => {
309 | |         $(
310 | |             impl $Ty {
...   |
367 | |                 #[stable(since = "1.63")]
...   |
497 | |     }
498 | | }
    | |_- in this expansion of `nonzero_unsigned_operations!`
    | |_- in this expansion of `nonzero_unsigned_operations!`
499 | 
500 | / nonzero_unsigned_operations! {
501 | |     NonZeroU8(u8);
502 | |     NonZeroU16(u16);
503 | |     NonZeroU32(u32);
506 | |     NonZeroUsize(usize);
507 | | }
    | |_- in this macro invocation


error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:432:17
    |
307 | / macro_rules! nonzero_unsigned_operations {
308 | |     ( $( $Ty: ident($Int: ident); )+ ) => {
309 | |         $(
310 | |             impl $Ty {
...   |
432 | |                 #[stable(since = "1.63")]
...   |
497 | |     }
498 | | }
    | |_- in this expansion of `nonzero_unsigned_operations!`
    | |_- in this expansion of `nonzero_unsigned_operations!`
499 | 
500 | / nonzero_unsigned_operations! {
501 | |     NonZeroU8(u8);
502 | |     NonZeroU16(u16);
503 | |     NonZeroU32(u32);
506 | |     NonZeroUsize(usize);
507 | | }
    | |_- in this macro invocation


error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:533:17
    |
510 | / macro_rules! nonzero_signed_operations {
511 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 | |         $(
513 | |             impl $Ty {
...   |
533 | |                 #[stable(since = "1.63")]
...   |
716 | |     }
717 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
    | |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | / nonzero_signed_operations! {
720 | |     NonZeroI8(i8) -> NonZeroU8(u8);
721 | |     NonZeroI16(i16) -> NonZeroU16(u16);
722 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation

error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:564:17
   --> library/core/src/num/nonzero.rs:564:17
    |
510 | / macro_rules! nonzero_signed_operations {
511 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 | |         $(
513 | |             impl $Ty {
...   |
564 | |                 #[stable(since = "1.63")]
...   |
716 | |     }
717 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
    | |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | / nonzero_signed_operations! {
720 | |     NonZeroI8(i8) -> NonZeroU8(u8);
721 | |     NonZeroI16(i16) -> NonZeroU16(u16);
722 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation

error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:599:17
   --> library/core/src/num/nonzero.rs:599:17
    |
510 | / macro_rules! nonzero_signed_operations {
511 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 | |         $(
513 | |             impl $Ty {
...   |
599 | |                 #[stable(since = "1.63")]
...   |
716 | |     }
717 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
    | |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | / nonzero_signed_operations! {
720 | |     NonZeroI8(i8) -> NonZeroU8(u8);
721 | |     NonZeroI16(i16) -> NonZeroU16(u16);
722 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation

error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:638:17
   --> library/core/src/num/nonzero.rs:638:17
    |
510 | / macro_rules! nonzero_signed_operations {
511 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 | |         $(
513 | |             impl $Ty {
...   |
638 | |                 #[stable(since = "1.63")]
...   |
716 | |     }
717 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
    | |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | / nonzero_signed_operations! {
720 | |     NonZeroI8(i8) -> NonZeroU8(u8);
721 | |     NonZeroI16(i16) -> NonZeroU16(u16);
722 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation

error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:672:17
   --> library/core/src/num/nonzero.rs:672:17
    |
510 | / macro_rules! nonzero_signed_operations {
511 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 | |         $(
513 | |             impl $Ty {
...   |
672 | |                 #[stable(since = "1.63")]
...   |
716 | |     }
717 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
    | |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | / nonzero_signed_operations! {
720 | |     NonZeroI8(i8) -> NonZeroU8(u8);
721 | |     NonZeroI16(i16) -> NonZeroU16(u16);
722 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation

error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:706:17
   --> library/core/src/num/nonzero.rs:706:17
    |
510 | / macro_rules! nonzero_signed_operations {
511 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 | |         $(
513 | |             impl $Ty {
...   |
706 | |                 #[stable(since = "1.63")]
...   |
716 | |     }
717 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
    | |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | / nonzero_signed_operations! {
720 | |     NonZeroI8(i8) -> NonZeroU8(u8);
721 | |     NonZeroI16(i16) -> NonZeroU16(u16);
722 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation

error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:754:17
   --> library/core/src/num/nonzero.rs:754:17
    |
729 | / macro_rules! nonzero_unsigned_signed_operations {
730 | |     ( $( $signedness:ident $Ty: ident($Int: ty); )+ ) => {
731 | |         $(
732 | |             impl $Ty {
...   |
754 | |                 #[stable(since = "1.63")]
...   |
915 | |     }
916 | | }
    | |_- in this expansion of `nonzero_unsigned_signed_operations!`
    | |_- in this expansion of `nonzero_unsigned_signed_operations!`
...
928 | / nonzero_unsigned_signed_operations! {
929 | |     unsigned NonZeroU8(u8);
930 | |     unsigned NonZeroU16(u16);
931 | |     unsigned NonZeroU32(u32);
940 | |     signed NonZeroIsize(isize);
941 | | }
    | |_- in this macro invocation


error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:789:17
    |
729 | / macro_rules! nonzero_unsigned_signed_operations {
730 | |     ( $( $signedness:ident $Ty: ident($Int: ty); )+ ) => {
731 | |         $(
732 | |             impl $Ty {
...   |
789 | |                 #[stable(since = "1.63")]
...   |
915 | |     }
916 | | }
    | |_- in this expansion of `nonzero_unsigned_signed_operations!`
    | |_- in this expansion of `nonzero_unsigned_signed_operations!`
...
928 | / nonzero_unsigned_signed_operations! {
929 | |     unsigned NonZeroU8(u8);
930 | |     unsigned NonZeroU16(u16);
931 | |     unsigned NonZeroU32(u32);
940 | |     signed NonZeroIsize(isize);
941 | | }
    | |_- in this macro invocation


error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:861:17
    |
729 | / macro_rules! nonzero_unsigned_signed_operations {
730 | |     ( $( $signedness:ident $Ty: ident($Int: ty); )+ ) => {
731 | |         $(
732 | |             impl $Ty {
...   |
861 | |                 #[stable(since = "1.63")]
...   |
915 | |     }
916 | | }
    | |_- in this expansion of `nonzero_unsigned_signed_operations!`
    | |_- in this expansion of `nonzero_unsigned_signed_operations!`
...
928 | / nonzero_unsigned_signed_operations! {
929 | |     unsigned NonZeroU8(u8);
930 | |     unsigned NonZeroU16(u16);
931 | |     unsigned NonZeroU32(u32);
940 | |     signed NonZeroIsize(isize);
941 | | }
    | |_- in this macro invocation


error[E0546]: missing 'feature'
   --> library/core/src/num/nonzero.rs:904:17
    |
729 | / macro_rules! nonzero_unsigned_signed_operations {
730 | |     ( $( $signedness:ident $Ty: ident($Int: ty); )+ ) => {
731 | |         $(
732 | |             impl $Ty {
...   |
904 | |                 #[stable(since = "1.63")]
...   |
915 | |     }
916 | | }
    | |_- in this expansion of `nonzero_unsigned_signed_operations!`
    | |_- in this expansion of `nonzero_unsigned_signed_operations!`
...
928 | / nonzero_unsigned_signed_operations! {
929 | |     unsigned NonZeroU8(u8);
930 | |     unsigned NonZeroU16(u16);
931 | |     unsigned NonZeroU32(u32);
940 | |     signed NonZeroIsize(isize);
941 | | }
    | |_- in this macro invocation


error: associated function has missing stability attribute
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


error: associated function has missing stability attribute
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


error: associated function has missing stability attribute
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


error: associated function has missing stability attribute
   --> library/core/src/num/nonzero.rs:537:17
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl $Ty {
...    |
537 | /|                 pub const fn abs(self) -> $Ty {
538 | ||                     // SAFETY: This cannot overflow to zero.
539 | ||                     unsafe { $Ty::new_unchecked(self.get().abs()) }
    | ||_________________^
...    |
716 |  |     }
717 |  | }
717 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | /  nonzero_signed_operations! {
720 | |      NonZeroI8(i8) -> NonZeroU8(u8);
721 | |      NonZeroI16(i16) -> NonZeroU16(u16);
722 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
726 | |  }

error: associated function has missing stability attribute
   --> library/core/src/num/nonzero.rs:568:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl $Ty {
...    |
568 | /|                 pub const fn checked_abs(self) -> Option<$Ty> {
569 | ||                     if let Some(nz) = self.get().checked_abs() {
570 | ||                         // SAFETY: absolute value of nonzero cannot yield zero values.
571 | ||                         Some(unsafe { $Ty::new_unchecked(nz) })
574 | ||                     }
575 | ||                 }
    | ||_________________^
...    |
...    |
716 |  |     }
717 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | /  nonzero_signed_operations! {
720 | |      NonZeroI8(i8) -> NonZeroU8(u8);
721 | |      NonZeroI16(i16) -> NonZeroU16(u16);
722 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
726 | |  }

error: associated function has missing stability attribute
   --> library/core/src/num/nonzero.rs:603:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl $Ty {
...    |
603 | /|                 pub const fn overflowing_abs(self) -> ($Ty, bool) {
604 | ||                     let (nz, flag) = self.get().overflowing_abs();
605 | ||                     (
606 | ||                         // SAFETY: absolute value of nonzero cannot yield zero values.
609 | ||                     )
610 | ||                 }
    | ||_________________^
...    |
...    |
716 |  |     }
717 |  | }
    |  |_- in this expansion of `nonzero_signed_operations!`
718 | 
719 | /  nonzero_signed_operations! {
720 | |      NonZeroI8(i8) -> NonZeroU8(u8);
721 | |      NonZeroI16(i16) -> NonZeroU16(u16);
722 | |      NonZeroI32(i32) -> NonZeroU32(u32);
...   |
725 | |      NonZeroIsize(isize) -> NonZeroUsize(usize);
726 | |  }

error: associated function has missing stability attribute
   --> library/core/src/num/nonzero.rs:642:17
    |
    |
510 |  / macro_rules! nonzero_signed_operations {
511 |  |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 |  |         $(
513 |  |             impl $Ty {
...    |
642 | /|                 pub const fn saturating_abs(self) -> $Ty {
643 | ||                     // SAFETY: absolute value of nonzero cannot yield zero values.
644 | ||                     unsafe { $Ty::new_unchecked(self.get().saturating_abs()) }
    | ||_________________^
