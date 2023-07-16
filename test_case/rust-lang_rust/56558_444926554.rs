
    -Z                       unpretty=val -- Present the input source, unstable (and less-pretty) variants;
        valid types are any of the types for `--pretty`, as well as:
        `flowgraph=<nodeid>` (graphviz formatted flowgraph for node),
        `everybody_loops` (all function bodies replaced with `loop {}`),
        `hir` (the HIR), `hir,identified`, or
        `hir,typed` (HIR with types for each node).

