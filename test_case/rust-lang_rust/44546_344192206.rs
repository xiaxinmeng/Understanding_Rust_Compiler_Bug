rust
let mut sum = 0;
(0..10).map(|x| sum += x).collect::<()>();
println!("{}", sum);
