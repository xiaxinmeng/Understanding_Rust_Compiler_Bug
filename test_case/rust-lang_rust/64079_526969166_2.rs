rust
struct SomeType; // Either declared or imported one scope above 'inner'
mod inner {
    use super::SomeType;
}
