
pub struct Graph {
    // all impls have a parent; the "root" impls have as their parent the def_id
    // of the trait
    // allow one or more parents since an intersection impl has at least two parents
    parent: DefIdMap<Vec<DefId>>,

    // provide overlap edges, I'm still not sure if, for any given overlap, I should insert two nodes in the 
    // maps (Eg <impl1, impl2> and <impl2, impl1>)
    overlap: DefIdMap<DefId>,

    // the "root" impls are found by looking up the trait's def_id.
    children: DefIdMap<Children>,
}
