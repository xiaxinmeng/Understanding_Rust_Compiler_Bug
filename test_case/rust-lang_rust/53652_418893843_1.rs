rust
pub fn copy_in_place(&mut self, src: Range<usize>, dest: RangeFrom<usize>) where T: Copy {
   assert!(src.end >= src.start);
   let count = src.end - src.start;
   // …
}
