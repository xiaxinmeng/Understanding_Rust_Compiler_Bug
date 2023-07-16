rust
        if base < ary.len() {
            if cmp == Equal { Ok(base) } else { Err(base + (cmp == Less) as usize) }
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
