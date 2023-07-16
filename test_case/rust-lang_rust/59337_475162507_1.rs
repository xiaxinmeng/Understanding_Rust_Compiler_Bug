
error[E0477]: the type `impl std::future::Future` does not fulfill the required lifetime
  --> src/main.rs:13:5
   |
13 |     foo(async move | f: &u8 | { *f });
   |     ^^^
   |
   = note: type must outlive any other region
