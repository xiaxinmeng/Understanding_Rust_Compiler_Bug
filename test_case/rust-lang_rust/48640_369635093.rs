`
   Compiling cargo-cache v0.1.0 (file:///tmp/cargo-cache)
     Running `rustc --crate-name cargo_cache src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=ff07950104b4651d -C extra-filename=-ff07950104b4651d --out-dir /tmp/cargo-cache/target/debug/deps -C incremental=/tmp/cargo-cache/target/debug/incremental -L dependency=/tmp/cargo-cache/target/debug/deps --extern walkdir=/tmp/cargo-cache/target/debug/deps/libwalkdir-5b91a5fbb37b0cac.rlib`
warning: unused import: `std::fs`
 --> src/lib.rs:4:5
  |
4 | use std::fs;
  |     ^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

warning: unused variable: `file2`
  --> src/lib.rs:20:6
   |
20 |     let file2 = WalkDir::new(format!("{}", dir.display()));
   |         ^^^^^ help: consider using `_file2` instead
   |
   = note: #[warn(unused_variables)] on by default

     Running `rustc --crate-name cargo_cache src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=e7b6b24911fa6523 -C extra-filename=-e7b6b24911fa6523 --out-dir /tmp/cargo-cache/target/debug/deps -C incremental=/tmp/cargo-cache/target/debug/incremental -L dependency=/tmp/cargo-cache/target/debug/deps --extern walkdir=/tmp/cargo-cache/target/debug/deps/libwalkdir-5b91a5fbb37b0cac.rlib --extern cargo_cache=/tmp/cargo-cache/target/debug/deps/libcargo_cache-ff07950104b4651d.rlib`
warning: unused import: `std::fs`
 --> src/lib.rs:4:5
  |
4 | use std::fs;
  |     ^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

warning: unused import: `lib::*`
 --> src/main.rs:6:5
  |
6 | use lib::*;
  |     ^^^^^^

warning: unused variable: `file2`
  --> src/lib.rs:20:6
   |
20 |     let file2 = WalkDir::new(format!("{}", dir.display()));
   |         ^^^^^ help: consider using `_file2` instead
   |
   = note: #[warn(unused_variables)] on by default

warning: function is never used: `cumulative_dir_size`
  --> src/lib.rs:11:1
   |
11 | pub fn cumulative_dir_size(dir: &PathBuf) -> u64 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 1.56 secs
