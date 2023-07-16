
error[E0283]: type annotations needed
 --> src/main.rs:2:69
  |
2 |     ["a", "b", "c"].iter().map(|elt| -> () { println!("{}", elt) }).collect();
  |                                                                     ^^^^^^^
  |                                                                     |
  |                                                                     cannot infer type for type parameter `B` declared on the method `collect`
  |                                                                     help: consider specifying the type argument in the method call: `collect::<B>`
  |
  = note: cannot resolve `_: std::iter::FromIterator<()>`
