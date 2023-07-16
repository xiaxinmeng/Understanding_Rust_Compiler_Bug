
error[E0391]: cycle detected when processing `test::{{opaque}}#0`
 --> <source>:7:14
  |
7 | fn test() -> impl TraitA {
  |              ^^^^^^^^^^^
  |

note: ...which requires processing `test`...
 --> <source>:7:26
  |
7 |   fn test() -> impl TraitA {
  |  __________________________^
8 | |     test()
9 | | }
  | |_^
  = note: ...which again requires processing `test::{{opaque}}#0`, completing the cycle

note: cycle used when checking item types in top-level module
 --> <source>:3:1
  |
3 | fn main() {
  | ^^^^^^^^^

error[E0720]: opaque type expands to a recursive type
 --> <source>:7:14
  |
7 | fn test() -> impl TraitA {
  |              ^^^^^^^^^^^ expands to self-referential type
  |
  = note: type resolves to itself


error: aborting due to 2 previous errors
