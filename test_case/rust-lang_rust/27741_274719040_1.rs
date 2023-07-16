rust
#[test]
fn test_excl_float_range() {
   let mut iter: ExclusiveRange<f32, f32> = ExclusiveRange {
       start: 0.0,
       stop: 1.0,
       step: 0.3,
   };
   assert_eq!(iter.next(), Some(0.0));
   assert_eq!(iter.next(), Some(0.3));
   assert_eq!(iter.next(), Some(0.6));
   assert_eq!(iter.next(), Some(0.9));
   assert_eq!(iter.next(), None);
}
