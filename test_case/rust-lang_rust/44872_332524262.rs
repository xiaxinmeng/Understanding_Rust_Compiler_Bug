
enum Where {
    /// We have not yet started dumping the control-flow graph, but we are about to.
    /// This is e.g. where the types of local variables appear and so forth.
    BeforeCFG,

    /// We just finished dumping the control-flow graph. This is right before the end of the file.
    AfterCFG,

    /// We are about to start dumping the given basic block.
    BeforeBlock(BasicBlockIndex),

    /// We are just about to dump the given statement or terminator.
    InCFG(Location),
}
