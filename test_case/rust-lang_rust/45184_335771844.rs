Rust
enum TerminatorKind {
// ...
    FalseEdges {
        real_target: BasicBlock,
        imaginary_targets: Vec<BasicBlock>
    }
}
