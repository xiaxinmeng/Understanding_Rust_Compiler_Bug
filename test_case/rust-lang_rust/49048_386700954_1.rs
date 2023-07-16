rust
let q = self / rhs;
let q = if self % rhs < 0 {
    if rhs > 0 { q - 1 } else { q + 1 }
} else {
    q
};
let r = self % rhs;
let r = if r < 0 {
    if rhs < 0 {
        r - rhs
    } else {
        r + rhs
    }
} else {
    r
};
(q, r)
