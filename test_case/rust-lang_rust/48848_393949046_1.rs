
warning: unused attribute
 --> src/main.rs:1:12
  |
1 | struct Foo<#[cfg(not(unix))] H>(H);
  |            ^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_attributes)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.65s
     Running `target/debug/playground`
