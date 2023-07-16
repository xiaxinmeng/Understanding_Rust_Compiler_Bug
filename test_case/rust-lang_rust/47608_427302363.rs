rust
#[derive(Serialize)]
#[serde(untagged)]
enum CellIndex {
    Auto,
    Index(u32),
}
