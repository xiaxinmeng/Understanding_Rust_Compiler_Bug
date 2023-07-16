rust
fn main3() {
    let vec1 = [1,2,3,4,5].into_iter().collect();
    let x = <_ as core::ops::Index<usize>>::index(&vec1, 0);
    let _vec2 : &Vec<usize> = &vec1;
}
