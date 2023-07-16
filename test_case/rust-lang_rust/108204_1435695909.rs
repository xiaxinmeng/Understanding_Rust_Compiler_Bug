rust
fn main() {
    let vec1 = [1, 2, 3].into_iter().collect();
    let x = Index::index(&vec1, 0);
    let _vec2: &Vec<usize> = &vec1;
}
