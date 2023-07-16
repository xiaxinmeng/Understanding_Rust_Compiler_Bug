
$ python ../rust/src/bootstrap/bootstrap.py --stage 2 --step dist --host arm-unknown-linux-gnueabihf
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> arm-unknown-linux-gnueabihf)
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> arm-unknown-linux-gnueabihf)
Building stage2 std artifacts (arm-unknown-linux-gnueabihf -> arm-unknown-linux-gnueabihf)
failed to run `rustc` to learn about target-specific information

To learn more, run the command again with --verbose.


command did not execute successfully: "/buildslave/build/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "8" "--target" "arm-unknown-linux-gnueabihf" "--release" "--features" " jemalloc" "--manifest-path" "/buildslave/rust/src/rustc/std_shim/Cargo.toml"
expected success, got: exit code: 101
