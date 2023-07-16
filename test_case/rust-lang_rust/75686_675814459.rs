rust
let v = vec![1usize, 2, 3];
let cow  = Cow::from(v);
let also_a_cow = Cow::Borrowed(&*cow);
