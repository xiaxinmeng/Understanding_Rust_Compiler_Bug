rust
#[non_exhaustive]
enum ProcThreadAttribute {
    ParentProcess(&Command),
    // other attributes we could use safely ...
}
