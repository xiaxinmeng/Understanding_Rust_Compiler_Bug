
struct Tree {
    mut leftTree: Option<@Tree>,
    mut rightTree: Option<@Tree>,
    key: int
}
