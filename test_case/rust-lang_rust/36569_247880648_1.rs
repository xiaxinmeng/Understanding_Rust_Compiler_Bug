 rust
(0..10).flat_map(|mut i| (0..10).map(move |j| { i += j; i }));  // this is fine
