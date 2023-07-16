rust
let mut i = 0;
let c = || i;
i += 1;
println!("{}", c());
