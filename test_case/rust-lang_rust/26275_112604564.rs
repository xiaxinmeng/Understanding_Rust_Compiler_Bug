
src/visit.rs:236:6: 236:7 error: the type parameter `G` is not constrained by the impl trait, self type, or predicates [E0207]
src/visit.rs:236 impl<G: Visitable> Dfs<G::NodeId, G::Map>
