
$ RUSTC=./stdin_empty.sh cargo +1.22 check
warning: unused manifest key: package.edition
   Compiling inner v0.1.0 (file:///home/jnelson/rust-lang/test-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10 secs
$ RUSTC=./stdin_empty.sh cargo +stable check
    Checking inner v0.1.0 (/home/jnelson/rust-lang/test-rust)
error: could not compile `inner`
$ RUSTC=./stdin_empty.sh cargo +beta check
    Checking inner v0.1.0 (/home/jnelson/rust-lang/test-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
