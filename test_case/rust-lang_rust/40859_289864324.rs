
rustc 1.16.0 (30cf806ef 2017-03-10)
error: cannot borrow immutable borrowed content `*var` as mutable
 --> <anon>:2:14
  |
1 | fn m(var: &[&mut u8]) {
  |           ---------- use `&mut [&mut mut u8]` here to make mutable
2 |     for i in var.iter_mut() {
  |              ^^^ cannot borrow as mutable

error: aborting due to previous error
