rust
stack.extend(
  mir.predecessor_locations(l)
      .map(|predecessor| {
          // There is an edge `predecessor -> l`. This is a backedge if `l` dominates `predecessor`.
          let is_backedge = l.dominates(predecessor, &self.dominators);
          (predecessor, IsBackEdge(is_backedge))
        }
);
