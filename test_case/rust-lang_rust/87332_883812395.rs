rust
pub enum Enum {
    Variant1 { x: i32, y: String },
    Variant2(Vec<u32>, std::path::PathBuf),
}
