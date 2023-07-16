rust
    for (bb, _) in traversal::reverse_postorder(&mir) {
        fx.llbb(bb);
    }
