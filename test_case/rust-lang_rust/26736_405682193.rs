rust
let arr = [Box::new(1), Box::new(2), Box::new(3)];
let (a, b, c) = (|[a, b, c]: [Box<i32>; 3]| (a, b, c))(arr);
