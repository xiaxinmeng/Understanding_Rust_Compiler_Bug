plain
+ python3 /checkout/x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --llvm-profile-generate=/tmp/tmp-pgo/llvm-pgo/prof-%p
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.07s

Option 'llvm-profile-generate' does not take an argument
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
  left: `1`,
 right: `2`: x.py should be run with `--stage 2` on CI, but was run with `--stage 1`', config.rs:1381:21
Build completed unsuccessfully in 0:00:00
