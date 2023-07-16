
println!("{:?}", vec![1, 2, 3].into_iter().collect::<Vec<usize>());
println!("{:?}", vec![1, 2, 3].into_iter().collect::<<<Vec<usize>>());
println!("{:?}", vec![1, 2, 3].into_iter().collect::<<<Vec<usize>>>());
println!("{:?}", vec![1, 2, 3].into_iter().collect::<<Vec<usize>>>());
bar::<<T as Foo>::Output();
