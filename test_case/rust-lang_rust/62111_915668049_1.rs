rust
fn abs_diff(slf: U, other: U)  -> U {
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}
