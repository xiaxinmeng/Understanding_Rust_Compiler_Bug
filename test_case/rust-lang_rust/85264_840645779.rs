rust
if VAL <= 3 {
    // The call to intrinsic is still emitted with an out-of-range value
    intrinsic(VAL);
} else {
    panic!();
}
