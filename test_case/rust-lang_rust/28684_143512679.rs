 rust
#[derive(Debug)] struct RefMut<'a>(&'a mut i32);
#[derive(Debug)] struct RefImmut<'a>(&'a i32);

let ref_mut = RefMut(&mut 9);
let ref_immut = RefImmut(&9);

println!("{:?}", ref_mut);
println!("{:?}", ref_immut);
