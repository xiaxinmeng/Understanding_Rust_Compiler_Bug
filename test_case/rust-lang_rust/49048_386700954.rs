rust
let (q, r) = (self / rhs, self % rhs);
if r < 0 {
    if rhs > 0 {
        (q - 1, r - rhs)
    } else {
        (q + 1, r + rhs)
    }
} else {
    (q, r)
}
