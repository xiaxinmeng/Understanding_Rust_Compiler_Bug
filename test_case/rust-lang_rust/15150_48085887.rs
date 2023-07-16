 rust
match cx.tcx.named_region_map.find(&lifetime.id) {
    None => cx.sess().span_bug("found unknown lifetime during linting"),
    Some(resolve_lifetime::DefStaticRegion) => { /* nothing to do */ }
    Some(resolve_lifetime::DefEarlyBoundRegion(_, _, id)) |
    Some(resolve_lifetime::DefLateBoundRegion(_, _, id)) |
    Some(resolve_lifetime::DefFreeRegion(_,id)) => {
        self.checked.insert(id)
    }
}
