rust
enum UnwindStrategy {
    Resume,
    Unreachable,
    Goto(BasicBlock),
}
