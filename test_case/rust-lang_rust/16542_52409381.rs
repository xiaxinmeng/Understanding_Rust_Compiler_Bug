
let mut a = BitvSet::from_bitv(bitv::from_bytes([0b00000000]));
let b = BitvSet::new();
println!("{}", a == b);
