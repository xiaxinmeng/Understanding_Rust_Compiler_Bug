 rust
let x = vec![0, 1, 2, 3];
let y = &x as *const Vec<i32>;
unsafe {
    let z = y.read();
    println!("{:?}", z); // output [0, 1, 2, 3]
}
