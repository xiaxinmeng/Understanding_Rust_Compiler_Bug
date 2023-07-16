 rust
v.iter().min_by(|v| a);
v.iter().min_by(|v| &*a);
