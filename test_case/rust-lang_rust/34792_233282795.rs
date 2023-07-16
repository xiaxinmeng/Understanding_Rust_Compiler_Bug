 rust
                // Arbitrarily give param candidates priority
                // over projection and object candidates.
                ObjectCandidate => true,
                ProjectionCandidate(_) => {
                    if let ProjectionCandidate(_) = other.candidate {
                        false
                    } else {
                        true
                    }
                },
