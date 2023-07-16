 rust
let it = range(100, 999).collect::<Vec<int>>();
    println!("{}",
        it.move_iter().flat_map(|x| range(x,999).collect::<Vec<int>>()
                                                .iter()
                                                .map(|j| j * x))
                    .filter(|x| x.to_str() == x.to_str().as_slice().chars().rev().collect::<String>())
                    .max_by(|x|x.to_int())
        );
