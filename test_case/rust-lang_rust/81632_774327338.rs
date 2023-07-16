
❯ cargo build --target=aarch64-apple-ios-sim
   Compiling testing v0.1.0 (/Users/rth/code/experiments/testing)
    Finished dev [unoptimized + debuginfo] target(s) in 1.88s

❯ file testing
testing: Mach-O 64-bit executable arm64

target/aarch64-apple-ios-sim/debug on  master [?]
❯ ./testing
dyld: attempt to run simulator program outside simulator (DYLD_ROOT_PATH not set)
zsh: abort      ./testing
