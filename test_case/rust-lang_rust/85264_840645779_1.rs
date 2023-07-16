rust
if VAL <= 3 {
    // LLVM asserts because the argument is not a constant
    intrinsic(VAL & 3);
} else {
    panic!();
}
