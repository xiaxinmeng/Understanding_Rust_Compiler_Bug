
error[E0282]: type annotations needed
  --> src/main.rs:16:7
   |
16 |     q.lol(||());
   |       ^^^
   |
help: try using a fully qualified path to specify the expected types
   |
16 |     <Qqq as MyTrait<T>>::lol::<[closure@src/main.rs:16:11: 16:13]>(&q, ||());
   |     ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ ~

error[E0283]: type annotations needed
  --> src/main.rs:16:7
   |
16 |     q.lol(||());
   |       ^^^
   |
note: multiple `impl`s satisfying `Qqq: MyTrait<_>` found
  --> src/main.rs:7:1
   |
7  | impl MyTrait<u32> for Qqq{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
...
10 | impl MyTrait<u64> for Qqq{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
help: try using a fully qualified path to specify the expected types
   |
16 |     <Qqq as MyTrait<T>>::lol::<[closure@src/main.rs:16:11: 16:13]>(&q, ||());
   |     ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ ~
