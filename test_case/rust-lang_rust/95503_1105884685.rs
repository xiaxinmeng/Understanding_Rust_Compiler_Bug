
$ cargo c -q -p inner --message-format short
/home/jnelson/rust-lang/test-rust/src/lib.rs:1:4: warning: function is never used: `unused`
$ cargo c -q -p outer --message-format short
/home/jnelson/rust-lang/test-rust/src/lib.rs:1:4: warning: function is never used: `unused`
$ cargo c -q -p inner -p outer --message-format short
/home/jnelson/rust-lang/test-rust/src/lib.rs:1:4: warning: function is never used: `unused`
