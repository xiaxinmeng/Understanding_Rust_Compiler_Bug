 Shell
> rustdoc src/sysinfo.rs --crate-name sysinfo -o /home/imperio/rust/sysinfo/target/doc -L dependency=/home/imperio/rust/sysinfo/target/debug -L dependency=/home/imperio/rust/sysinfo/target/debug/deps --extern libc=/home/imperio/rust/sysinfo/target/debug/deps/liblibc-9b7976990ae0dbd4.rlib --extern libc=/home/imperio/rust/sysinfo/target/debug/deps/liblibc-9b7976990ae0dbd4.rlib --output-format json
thread '<unnamed>' panicked at 'Rust generated JSON is invalid: SyntaxError(EOFWhileParsingObject, 1, 105198)', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustdoc/lib.rs:525
thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value: Any', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/result.rs:729
