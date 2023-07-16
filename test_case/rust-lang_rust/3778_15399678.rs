
struct Tree {
leftTree: @Option<Tree>,
rightTree: @Option<Tree>,
key: int
}

fn main() {
    let t = Tree {leftTree: @None, rightTree: @None, key: 0};
}
