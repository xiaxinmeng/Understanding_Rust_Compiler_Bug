rust
struct CargoDeb {
    pub variants: Option<HashMap<String, Box<CargoDeb>>>,
}
