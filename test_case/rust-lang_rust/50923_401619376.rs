
error[E0425]: cannot find value `Member` in this scope
 --> src/main.rs:6:5
  |
6 |     Member;
  |     ^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
  |
1 | use other_module_with_pub::Member;
1 | use other_module_with_private::Member; // Could be made `pub`
  |
