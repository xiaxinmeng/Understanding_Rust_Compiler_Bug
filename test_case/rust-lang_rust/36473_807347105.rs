
    -Z                       unpretty=val -- present the input source, unstable (and less-pretty) variants;
        valid types are any of the types for `--pretty`, as well as:
        `expanded`, `expanded,identified`,
        `expanded,hygiene` (with internal representations),
        `everybody_loops` (all function bodies replaced with `loop {}`),
        `ast-tree` (raw AST before expansion),
        `ast-tree,expanded` (raw AST after expansion),
        `hir` (the HIR), `hir,identified`,
        `hir,typed` (HIR with types for each node),
        `hir-tree` (dump the raw HIR),
        `mir` (the MIR), or `mir-cfg` (graphviz formatted MIR)
