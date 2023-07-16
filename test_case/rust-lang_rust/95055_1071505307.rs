
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
  [--> src/lib.rs:15:5
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)   |
12 | async fn main() -> Result<(), Box<dyn std::error::Error>> {
   |                    -------------------------------------- expected `Result<(), Box<(dyn std::error::Error + 'static)>>` because of return type
...
15 |     Ok(())
   |     ^^^^^^ expected enum `Result`, found `()`
   |
   = note:   expected enum `Result<(), Box<(dyn std::error::Error + 'static)>>`
           found unit type `()`
help: try adding an expression at the end of the block
   |
15 ~     Ok(());
16 +     Ok(())
   |
