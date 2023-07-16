
warning: function cannot return without recursing
 --> src/main.rs:3:1
  |
3 | fn silly<I: Iterator<Item=()>>(it: I) {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
4 |     silly(it.chain(iter::empty()))
  |     ------------------------------ recursive call site
  |
  = note: #[warn(unconditional_recursion)] on by default
  = help: a `loop` may express intention better if this is on purpose

error: reached the recursion limit while instantiating `silly::<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Empty<()>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>>`
 --> src/main.rs:3:1
  |
3 | / fn silly<I: Iterator<Item=()>>(it: I) {
4 | |     silly(it.chain(iter::empty()))
5 | | }
  | |_^
