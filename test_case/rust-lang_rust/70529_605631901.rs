rust
        } else {
            // No idea what the length is, which happens if we have e.g.,
            // `let [a, b] = arr` where `arr: [T; N]` where `const N: usize`.
            self.error_scrutinee_unfixed_length(span);
        }
