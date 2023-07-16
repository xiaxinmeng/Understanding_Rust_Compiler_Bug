
let len = array.len();
for i in 0..len {
    let min = (i..len).min_by_key(|x| array[*x]).unwrap();
    array.swap(min, i);
}
