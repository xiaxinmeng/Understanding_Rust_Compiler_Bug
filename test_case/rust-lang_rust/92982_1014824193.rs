rust
let filtered_vec = (0..50)
    .filter(|&x| x < 40)
    .collect_with(Vec::with_capacity(40));

// Instead of
let mut vec = Vec::with_capacity(40);
let filtered_vec2 = (0..50)
    .filter(|&x| x < 40)
    .collect_into(&mut vec);
