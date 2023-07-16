
if index < 3 {
    *unsafe_index(xs, index) = 100;
} else {
    fail_bounds_check(...);
}
