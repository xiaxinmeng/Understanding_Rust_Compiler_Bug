
/src/tmp/unreachable$ cargo run 
warning: unreachable expression
  --> src/main.rs:13:5
   |
11 |         terminate();
   |         ----------- any code following this expression is unreachable
12 |     }
13 |     println!("Hello, world!");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable expression
   |
   = note: `#[warn(unreachable_code)]` on by default
note: this expression has type `Never`, which is uninhabited
  --> src/main.rs:11:9
   |
11 |         terminate();
   |         ^^^^^^^^^^^
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `unreachable` (bin "unreachable") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/unreachable`
Hello, world!

