
error[E0282]: type annotations needed for `Result<Result<(), E>, ()>`
  --> a.rs:52:25
   |
52 |     proj.get_projected(|b| async {
   |                         ^
   |
help: consider giving this closure parameter an explicit type, where the type for type parameter `E` is specified
   |
52 |     proj.get_projected(|b: Result<Result<(), E>, ()>| async {
   |                          +++++++++++++++++++++++++++
