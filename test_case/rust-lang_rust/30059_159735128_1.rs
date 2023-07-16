
let v = vec![1, 3, 5, 7, 9];
let mut iter = v.iter();
while let Some(x) = iter.next() {
    println!("{}", x);
}
