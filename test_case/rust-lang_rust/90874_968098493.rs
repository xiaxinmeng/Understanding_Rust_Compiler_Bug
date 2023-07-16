plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:116:9
    |
101 | impl<Idx: Add<Idx> + Copy> Range<Idx> {
    |      --- this type parameter
...
116 |         (position+self.start)..(position+self.end)
    |         ^^^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Add>::Output`
    |
    |
101 | impl<Idx: Add<Idx> + Copy + Add<Output = Idx>> Range<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:116:32
    |
    |
101 | impl<Idx: Add<Idx> + Copy> Range<Idx> {
    |      --- this type parameter
...
116 |         (position+self.start)..(position+self.end)
    |                                ^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Add>::Output`
    |
    |
101 | impl<Idx: Add<Idx> + Copy + Add<Output = Idx>> Range<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:135:9
    |
    |
120 | impl<Idx: Sub<Idx> + Copy> Range<Idx> {
    |      --- this type parameter
...
135 |         (self.start-position)..(self.end-position)
    |         ^^^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Sub>::Output`
    |
    |
120 | impl<Idx: Sub<Idx> + Copy + Sub<Output = Idx>> Range<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:135:32
    |
    |
120 | impl<Idx: Sub<Idx> + Copy> Range<Idx> {
    |      --- this type parameter
...
135 |         (self.start-position)..(self.end-position)
    |                                ^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Sub>::Output`
    |
    |
120 | impl<Idx: Sub<Idx> + Copy + Sub<Output = Idx>> Range<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:257:9
    |
    |
242 | impl<Idx: Add<Idx> + Copy> RangeFrom<Idx> {
    |      --- this type parameter
...
257 |         (position+self.start)..
    |         ^^^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Add>::Output`
    |
    |
242 | impl<Idx: Add<Idx> + Copy + Add<Output = Idx>> RangeFrom<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:276:9
    |
    |
261 | impl<Idx: Sub<Idx> + Copy> RangeFrom<Idx> {
    |      --- this type parameter
276 |         (self.start-position)..
276 |         (self.start-position)..
    |         ^^^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Sub>::Output`
    |
    |
261 | impl<Idx: Sub<Idx> + Copy + Sub<Output = Idx>> RangeFrom<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:376:11
    |
    |
361 | impl<Idx: Add<Idx> + Copy> RangeTo<Idx> {
    |      --- this type parameter
...
376 |         ..(position+self.end)
    |           ^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Add>::Output`
    |
    |
361 | impl<Idx: Add<Idx> + Copy + Add<Output = Idx>> RangeTo<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:395:11
    |
    |
380 | impl<Idx: Sub<Idx> + Copy> RangeTo<Idx> {
    |      --- this type parameter
...
395 |         ..(self.end-position)
    |           ^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Sub>::Output`
    |
    |
380 | impl<Idx: Sub<Idx> + Copy + Sub<Output = Idx>> RangeTo<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:491:20
    |
    |
475 | impl<Idx: Add<Idx> + Copy> RangeInclusive<Idx> {
    |      --- this type parameter
...
491 |             start: position+self.start,
    |                    ^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Add>::Output`
    |
    |
475 | impl<Idx: Add<Idx> + Copy + Add<Output = Idx>> RangeInclusive<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:492:18
    |
    |
475 | impl<Idx: Add<Idx> + Copy> RangeInclusive<Idx> {
    |      --- this type parameter
...
492 |             end: position+self.end,
    |                  ^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Add>::Output`
    |
    |
475 | impl<Idx: Add<Idx> + Copy + Add<Output = Idx>> RangeInclusive<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:514:20
    |
    |
498 | impl<Idx: Sub<Idx> + Copy> RangeInclusive<Idx> {
    |      --- this type parameter
514 |             start: self.start-position,
514 |             start: self.start-position,
    |                    ^^^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Sub>::Output`
    |
    |
498 | impl<Idx: Sub<Idx> + Copy + Sub<Output = Idx>> RangeInclusive<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:515:18
    |
    |
498 | impl<Idx: Sub<Idx> + Copy> RangeInclusive<Idx> {
    |      --- this type parameter
...
515 |             end: self.end-position,
    |                  ^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Sub>::Output`
    |
    |
498 | impl<Idx: Sub<Idx> + Copy + Sub<Output = Idx>> RangeInclusive<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:778:12
    |
    |
763 | impl<Idx: Add<Idx> + Copy> RangeToInclusive<Idx> {
    |      --- this type parameter
...
778 |         ..=position+self.end
    |            ^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Add>::Output`
    |
    |
763 | impl<Idx: Add<Idx> + Copy + Add<Output = Idx>> RangeToInclusive<Idx> {

error[E0308]: mismatched types
   --> library/core/src/ops/range.rs:797:12
    |
    |
782 | impl<Idx: Sub<Idx> + Copy> RangeToInclusive<Idx> {
    |      --- this type parameter
...
797 |         ..=self.end-position
    |            ^^^^^^^^^^^^^^^^^ expected type parameter `Idx`, found associated type
    = note: expected type parameter `Idx`
    = note: expected type parameter `Idx`
              found associated type `<Idx as Sub>::Output`
    |
    |
782 | impl<Idx: Sub<Idx> + Copy + Sub<Output = Idx>> RangeToInclusive<Idx> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `core` due to 14 previous errors
Build completed unsuccessfully in 0:01:09
