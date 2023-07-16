rust
match size_of::<TypeId>() {
  8 => /* transmute_copy to u64 */,
  16 => /* transmute_copy to u128 */,
  _ => panic!(),
}
