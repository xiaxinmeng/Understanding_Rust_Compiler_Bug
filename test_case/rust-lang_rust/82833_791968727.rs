console
$ RUSTFLAGS="-Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code" cargo +ec7f258d543e1ac7d0b94435972331e85da8c509 run
   Compiling issue_82833 v0.1.0 (/Users/tous/Documents/Dev/rustc_issues/issue_82833)
    Finished dev [unoptimized + debuginfo] target(s) in 5.15s
     Running `target/debug/issue_82833`
[src/main.rs:8] ch = '😃'
[src/main.rs:10] string = "😃"

$ RUSTFLAGS="-Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code" cargo +409920873cf8a95739a55dc5fe5adb05e1b4758e run
   Compiling issue_82833 v0.1.0 (/Users/tous/Documents/Dev/rustc_issues/issue_82833)
    Finished dev [unoptimized + debuginfo] target(s) in 16.55s
     Running `target/debug/issue_82833`
[src/main.rs:8] ch = '😃'
[src/main.rs:10] string = "\u{0}\u{0}\u{0}\u{0}"
