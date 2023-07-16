
error[[E0282]](https://doc.rust-lang.org/nightly/error_codes/E0282.html): type annotations needed
  --> src/main.rs:11:9
   |
11 |         Ok(())
   |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
   |
help: consider specifying the generic arguments
   |
11 |         Ok::<(), E>(())
   |           +++++++++
