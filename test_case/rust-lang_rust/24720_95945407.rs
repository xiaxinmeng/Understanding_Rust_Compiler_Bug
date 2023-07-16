 rust
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[allow(unused_comparisons)]
            fn steps_between(start: &$t, end: &$t, by: &$t) -> Option<usize> {
                if *by == 0 { return None; }
                let mut start = *start;
                let mut end = *end;
                let mut by = *by;
                if by < 0 {
                    start *= -1;
                    end *= -1;
                    by *= -1;
                }
                if start <= end {
                    let diff = end - start;
                    if diff % by > 0 {
                        Some((diff / by) as usize + 1)
                    } else {
                        Some((diff / by) as usize)
                    }
                } else {
                    Some(0)
                }
            }
