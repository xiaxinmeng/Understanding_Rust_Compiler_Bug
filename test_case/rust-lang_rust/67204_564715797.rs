
error: `async` blocks are only available in 2018 edition
 --> src/main.rs:7:11
  |
7 |     let x = async {
  | ____________^
8 ||        bar()?;
9 ||    }; 
  ||____^
  |
  = note: edit your `Cargo.toml` file and add `edition = 2018` to the `[package]` section if you wish to use async/await in your crate
  = note: for more information, visit <APPROPRIATE BOOK SECTION>

error: aborting due the previous errors
