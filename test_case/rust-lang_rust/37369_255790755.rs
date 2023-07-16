
error[E0277]: the trait bound `{integer}: std::ops::Add<()>` is not satisfied
  --> file2.rs:13:9
   |
13 |      foo(1 + bar(x,
   | ┌────────┘ starting here...
14 | │            y),
   | └─────────────┘ ...ending here: trait `{integer}: std::ops::Add<()>` not satisfied
   |
