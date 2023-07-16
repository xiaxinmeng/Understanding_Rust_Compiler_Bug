rust
struct Tree {
    children: Vec<Tree>,
}

fn traverse(t: &Tree) {
    println!();
    for c in &t.children {
        traverse(c);
    }
}

fn main() {
    let t = Tree { children: Vec::new() };
    traverse(&t);
}
