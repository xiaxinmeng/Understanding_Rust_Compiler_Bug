
error[E0597]: borrowed value does not live long enough
 --> src/lib.rs:3:57
  |
3 | pub const Z: Cow<'static, [ [u8; 3] ]> = Cow::Borrowed(&[*b"ABC"]);
  |                                                         ^^^^^^^^^- temporary value only lives until here
  |                                                         |
  |                                                         temporary value does not live long enough
  |
  = note: borrowed value must be valid for the static lifetime...
