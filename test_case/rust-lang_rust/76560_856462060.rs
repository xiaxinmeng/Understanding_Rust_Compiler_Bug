rust
const fn my_cast(arg: u8) -> usize {
  arg as usize
}
struct Foo<const N: u8>([u8; my_cast(N)]);
