 rust
fn env<T: ToCStr>(self, k: T, v: T) -> Self;
fn env_remove<T: ToCStr>(self, k: T) -> Self;
fn env_override<T: ToCStr>(self, k: &[(T, T)]) -> Self;
