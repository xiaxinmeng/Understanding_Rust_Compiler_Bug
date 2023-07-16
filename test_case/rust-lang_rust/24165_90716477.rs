
pub trait Flatten<T> {
  /// Emit a value into a Vec of bytes.
  fn emit(v: &Self, dest: &mut Vec<u8>) -> Result<(), ()>;
  /// Parse a value out of a stream of bytes.
  fn read<'a>(s: &mut MemStream<'a>) -> Result<Self, EOF>;
}

#[macro_export]
macro_rules! flatten_unit_struct_impl(
  ...
);

#[macro_export]
macro_rules! flatten_struct_impl(
  ...
);

#[macro_export]
macro_rules! flatten_enum_impl(
  ...
);

impl<T> Flatten for T where T: Copy {
  ...
}

impl<T> Flatten for Vec<T> where T: Copy {
  ...
}

impl Flatten for String {
  ...
}
