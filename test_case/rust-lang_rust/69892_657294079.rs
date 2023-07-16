
error[E0433]: failed to resolve: could not find `main` in `tokio`
  --> src/main.rs:17:10
   |
17 | #[tokio::main]
   |          ^^^^ could not find `main` in `tokio`

error[E0308]: mismatched types
  --> src/main.rs:26:64
   |
26 | async fn handle_get_data() -> Result<warp::reply::Json, Error> {}
   |                                                                ^^ expected enum `std::result::Result`, found `()`
   |
   = note:   expected enum `std::result::Result<warp::reply::Json, std::io::Error>`
           found unit type `()`

error[E0277]: `main` has invalid return type `impl core::future::future::Future`
  --> src/main.rs:18:20
   |
18 | async fn main() -> Result<(), Error> {
   |                    ^^^^^^^^^^^^^^^^^ `main` can only return types that implement `std::process::Termination`
   |
   = help: consider using `()`, or a `Result`

error[E0752]: `main` function is not allowed to be `async`
  --> src/main.rs:18:1
   |
18 | async fn main() -> Result<(), Error> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`

error: aborting due to 4 previous errors
