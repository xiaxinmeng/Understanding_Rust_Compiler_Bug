rust
fn abs_diff(slf: I, other: I)  -> U {
    if slf < other {
        (other as U).wrapping_sub(slf as U)
    } else {
        (slf as U).wrapping_sub(other as U)
    }
}
