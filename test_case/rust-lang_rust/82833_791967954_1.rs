console
$ RUSTFLAGS="-Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code" cargo +nightly run
   Compiling issue_82833 v0.1.0 (/Users/tous/Documents/Dev/rustc_issues/issue_82833)
    Finished dev [unoptimized + debuginfo] target(s) in 46.25s
     Running `target/debug/issue_82833`
[src/main.rs:8] ch = '😃'
[src/main.rs:10] string = "\u{0}\u{0}\u{0}\u{0}"