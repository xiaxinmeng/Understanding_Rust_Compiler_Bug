rust
/// Node of the function graph.
#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum ControlFlowTarget {
    /// A basic block
    Resolved(BasicBlock),
    /// An unresolved indirect jump
    Unresolved(Rvalue),
    /// An error occured while disassembling
    Failed(u64, Cow<'static, str>),
}

/// Graph of basic blocks and jumps
pub type ControlFlowGraph = AdjacencyList<ControlFlowTarget, Guard>;
