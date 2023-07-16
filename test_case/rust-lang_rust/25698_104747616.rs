 rust
let num = [[1, 2], [3, 4], [5, 6]];
let v = num.into_iter()
           .flat_map(|a| a.into_iter() )
           .collect::<Vec<i32>>();
println!("{:?}", v);
