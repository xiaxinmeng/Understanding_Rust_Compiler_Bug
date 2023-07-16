rust
// General ifs to do the saturation
if x.is_nan() {
    0
} else if x >= 0x1.fffffep30 {
    2147483647
} else if x >= -0x1p31 {
    // Protection against trapping
    if x.abs() < 0x1p31 {
        (int)x
    } else {
        -2147483648
    }
} else {
    -2147483648
}
