
jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ uname -a
Linux scm-arm1 5.0.0-38-generic #41-Ubuntu SMP Tue Dec 3 00:29:46 UTC 2019 aarch64 GNU/Linux

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812 $ git clone https://github.com/berkus/mre-write_to.git
Cloning into 'mre-write_to'...
remote: Enumerating objects: 9, done.
remote: Counting objects: 100% (9/9), done.
remote: Compressing objects: 100% (8/8), done.
remote: Total 9 (delta 0), reused 9 (delta 0), pack-reused 0
Unpacking objects: 100% (9/9), done.

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812 $ cd mre-write_to/

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ cargo build
   Compiling test-write-to v0.1.0 (/home/jagran01/work/projects/rustc-tickets/68812/mre-write_to)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src/main.rs:2:1
  |
2 | #![feature(format_args_nl)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0554`.
error: could not compile `test-write-to`.

To learn more, run the command again with --verbose.

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ echo nightly > rust-toolchain

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ ~/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/bin/rustc --version
rustc 1.47.0-nightly (81e754c35 2020-08-02)

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ cargo build
   Compiling test-write-to v0.1.0 (/home/jagran01/work/projects/rustc-tickets/68812/mre-write_to)
    Finished dev [unoptimized + debuginfo] target(s) in 1.05s

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/test-write-to`
Converting to str
Converted to str
Hello, world!

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running target/debug/deps/test_write_to-d86fea7b5f3da2c4

running 1 test
Converting to str
test tests::write_to_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ cargo clean

jagran01@scm-arm1 ~/work/projects/rustc-tickets/68812/mre-write_to $ cargo build --verbose
   Compiling test-write-to v0.1.0 (/home/jagran01/work/projects/rustc-tickets/68812/mre-write_to)
     Running `rustc --crate-name test_write_to --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C panic=abort -Cembed-bitcode=no -C debuginfo=2 -C metadata=50ae77b73baa9539 -C extra-filename=-50ae77b73baa9539 --out-dir /home/jagran01/work/projects/rustc-tickets/68812/mre-write_to/target/debug/deps -C incremental=/home/jagran01/work/projects/rustc-tickets/68812/mre-write_to/target/debug/incremental -L dependency=/home/jagran01/work/projects/rustc-tickets/68812/mre-write_to/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 1.00s
