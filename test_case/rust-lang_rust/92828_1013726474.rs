rust
enum UnwindAction {
    None,
    Terminate,
    Cleanup(BasicBlock),
    // LSDA can also encode Handler(catch) but we only use them for the r#try intrinsic so it's not needed here
}
