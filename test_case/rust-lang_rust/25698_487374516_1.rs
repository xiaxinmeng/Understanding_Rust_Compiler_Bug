rust
fn main() {
    let num = [[1, 2], [3, 4], [5, 6]];
    let v = num.into_iter()
               .flat_map(|a| a.into_iter() )
               .map(|it| it.clone())        // copy each element
               .collect::<Vec<_>>();        // note: Vec<_>
    println!("{:?}", v);
    
    assert_eq!(vec![1, 2, 3, 4, 5, 6], v);
}
