 rust
let values: Vec<u8> = vec![1,2,3,4,5,6,7,8];

for [x,y,z] in values.as_slice().chunks(3).filter(|&xs| xs.len() == 3) {
    println!("y={}", y);
}
