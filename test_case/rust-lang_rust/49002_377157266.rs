
$ cargo new foo
$ cd foo
$ echo 'pretty_env_logger = "*"' >> Cargo.toml
$ cargo doc -p pretty_env_logger
...
error[E0465]: multiple rmeta candidates for `ansi_term` found
  --> /home/danilo/.cargo/registry/src/github.com-1ecc6299db9ec823/pretty_env_logger-0.2.2/src/lib.rs:24:1
   |
24 | extern crate ansi_term;
   | ^^^^^^^^^^^^^^^^^^^^^^^
   |
note: candidate #1: /tmp/foo/target/debug/deps/libansi_term-8f29ff0b2086d6a6.rmeta
  --> /home/danilo/.cargo/registry/src/github.com-1ecc6299db9ec823/pretty_env_logger-0.2.2/src/lib.rs:24:1
   |
24 | extern crate ansi_term;
   | ^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2: /tmp/foo/target/debug/deps/libansi_term-fa0b3ae5de1b2f52.rmeta
  --> /home/danilo/.cargo/registry/src/github.com-1ecc6299db9ec823/pretty_env_logger-0.2.2/src/lib.rs:24:1
   |
24 | extern crate ansi_term;
   | ^^^^^^^^^^^^^^^^^^^^^^^

error[E0463]: can't find crate for `ansi_term`
  --> /home/danilo/.cargo/registry/src/github.com-1ecc6299db9ec823/pretty_env_logger-0.2.2/src/lib.rs:24:1
   |
24 | extern crate ansi_term;
   | ^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to 2 previous errors

error: Could not document `pretty_env_logger`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name pretty_env_logger /home/danilo/.cargo/registry/src/github.com-1ecc6299db9ec823/pretty_env_logger-0.2.2/src/lib.rs -o /tmp/foo/target/doc -L dependency=/tmp/foo/target/debug/deps --extern ansi_term=/tmp/foo/target/debug/deps/libansi_term-fa0b3ae5de1b2f52.rmeta --extern env_logger=/tmp/foo/target/debug/deps/libenv_logger-bd42acbe717fae2e.rmeta --extern ansi_term=/tmp/foo/target/debug/deps/libansi_term-8f29ff0b2086d6a6.rmeta --extern log=/tmp/foo/target/debug/deps/liblog-c6a51ef82288204a.rmeta` (exit code: 101)
