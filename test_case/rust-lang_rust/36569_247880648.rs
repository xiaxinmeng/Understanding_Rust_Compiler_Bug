 rust
(0..10).flat_map(|_| { let s = String::new(); Some(1).map(|_| s) });
