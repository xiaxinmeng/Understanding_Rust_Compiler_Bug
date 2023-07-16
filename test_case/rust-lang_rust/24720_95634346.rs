 rust
             fn steps_between(start: &$t, end: &$t, by: &$t) -> Option<usize> {
                 if *start <= *end {
                     Some(((*end - *start) / *by) as usize)
                 } else {
                     Some(0)
                 }
             }
