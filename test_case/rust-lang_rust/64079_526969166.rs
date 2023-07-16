rust
struct TopLevelStruct;
mod outer {
    struct OuterStruct;
    mod inner {
        use super::OuterStruct;
        use super::super::TopLevelStruct;
    }
}
