text
error[E0391]: cycle detected when const checking if rvalue is promotable to static `V`
 --> src/lib.rs:1:1
  |
1 | const V: Vec<u8> = {
  | ^^^^^^^^^^^^^^^^^^
  |
note: ...which requires checking which parts of `V` are promotable to static...
 --> src/lib.rs:3:16
  |
3 |     res.extend(V.iter());
  |                ^
  = note: ...which again requires const checking if rvalue is promotable to static `V`, completing the cycle
  = note: cycle used when running analysis passes on this crate
