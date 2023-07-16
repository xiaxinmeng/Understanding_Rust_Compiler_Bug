
error[E0283]: type annotations required: cannot resolve `_: std::iter::FromIterator<()>`
  --> src/main.rs:83:65
   |
83 | ["a", "b", "c"].iter().map(|elt| -> () { println!("{}", elt) }).collect();
   |                                                                 ^^^^^^^
