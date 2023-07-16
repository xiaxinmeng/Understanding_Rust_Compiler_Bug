
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
 --> test.rs:7:11
  |
7 |         | x).collect::<Vec<_>>();
  |           ^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the body at 7:10...
 --> test.rs:7:11
  |
7 |         | x).collect::<Vec<_>>();
  |           ^
note: ...so that expression is assignable (expected &u32, found &u32)
 --> test.rs:7:11
  |
7 |         | x).collect::<Vec<_>>();
  |           ^
note: but, the lifetime must be valid for the method call at 4:21...
 --> test.rs:4:22
  |
4 |       let references = nodes.iter().map(
  |  ______________________^ starting here...
5 | |         |x
6 | |         : &u32 // If this line is uncommented I get lifetime problems
7 | |         | x).collect::<Vec<_>>();
  | |________________________________^ ...ending here
note: ...so type `fn(std::iter::Map<std::slice::Iter<'_, u32>, [closure@test.rs:5:9: 7:12]>) -> std::vec::Vec<&u32> {<std::iter::Map<std::slice::Iter<'_, u32>, [closure@test.rs:5:9: 7:12]> as std::iter::Iterator>::collect::<std::vec::Vec<&u32>>}` of expression is valid during the expression
 --> test.rs:4:22
  |
4 |       let references = nodes.iter().map(
  |  ______________________^ starting here...
5 | |         |x
6 | |         : &u32 // If this line is uncommented I get lifetime problems
7 | |         | x).collect::<Vec<_>>();
  | |________________________________^ ...ending here

error: aborting due to previous error

