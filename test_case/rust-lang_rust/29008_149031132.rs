
use std::path::Path;
use std::collections::HashSet;
fn main() {
    let p1 = Path::new("/a/b/");
    let p2 = Path::new("/a/b/");
    let p3 = Path::new("/a/b");
    println!("{} {} {}", p1 == p2, p2 == p3, p1 == p3);
    let mut hs = HashSet::new();
    hs.insert(p1);
    hs.insert(p2);
    hs.insert(p3);
    println!("{}", hs.len());
}
