
error[E0277]: the trait bound `P: Append<i32>` is not satisfied
  --> file12.rs:19:5
   |
19 | /     fn process_item<P>(&mut self, partial: P)
20 | |     where
21 | |         P: Append<Self::Item>,
22 | |         P::Appended: Append<Self::Tail>,
   | |                                         - help: consider further restricting type parameter `P`: `, P: Append<i32>`
23 | |     {
24 | |
25 | |     }
   | |_____^ the trait `Append<i32>` is not implemented for `P`
