rust
if compiler.stage == 1 {
    cargo.rustflag("-Cinline-llvm=yes");
}
