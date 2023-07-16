rust
struct TopLevelStruct;
fn outer() {
    struct OuterStruct;
    fn inner() {
        use super::OuterStruct;
        use super::super::TopLevelStruct;
    }
}
