
src/visit.rs:278:33: 278:42 error: illegal recursive type; insert an enum or struct in the cycle, if this is desired [E0246]
src/visit.rs:278     G: for<'b> NeighborIter<'b, G::NodeId>,
