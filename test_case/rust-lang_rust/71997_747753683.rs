rust
// Bounds check not elided
pub fn cond_inline(s: &[bool; 2], cond: bool) -> bool {
    for i in (if cond { 0..=1 } else { 0..=0 }) {
        if s[i] {
            return true;
        }
    }

    false
}

// Bounds check elided
pub fn cond_outside(s: &[bool; 2], cond: bool) -> bool {
    if cond {
        for i in 0..=1 {
            if s[i] {
                return true;
            }
        }
    }
    else {
        for i in 0..=0 {
            if s[i] {
                return true;
            }
        }
    }

    false
}
