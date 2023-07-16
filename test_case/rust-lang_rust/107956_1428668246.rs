sh
## 1st test ##

# make sure this doesn't panic
x dist --stage 0

## 2nd test ##

# build stage 0
x build --stage 0

# then link the stage0-sysroot
rustup toolchain link {toolchain_name} {path_to_stage0-sysroot}

# check if rustc works properly with linked toolchain
rustc +{toolchain_name} --version

# sanity check for rustc, basically create simple rust project and run it with linked toolchain
cargo new hello
cd hello
cargo +{toolchain_name} run
