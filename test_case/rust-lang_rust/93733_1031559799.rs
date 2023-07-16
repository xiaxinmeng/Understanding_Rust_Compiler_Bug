rust
struct RefText<'txt> {
  ptr: &'txt mut Text<'txt>
}
