rust
#[non_exhaustive]
enum SomeEnum {
   A = 0,
   B = 255,
   C, // Discriminant is `u8` without this, `u16` with this. So it will break hashing as well.
}
