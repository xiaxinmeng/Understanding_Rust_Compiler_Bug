rust
let mut arc = Arc::new(0);
let clone = Arc::clone(&arc);
unsafe {
    *Arc::get_mut_unchecked(&mut arc) = 1;
}
if *clone == 1 {
    println!("1"); // this branch is always taken
} else {
    println!("0");
}
