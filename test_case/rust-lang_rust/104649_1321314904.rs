
error[E0282]: type annotations needed for `Result<Result<(), E>, ()>`
  --> a.rs:52:25
   |
52 |     proj.get_projected(|b| async {
   |                         ^ consider giving this closure parameter the explicit type `Result<Result<(), E>, ()>`, where the type parameter `E` is specified
