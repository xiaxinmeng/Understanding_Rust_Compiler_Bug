rust
let a = (1, vec![1, 2, 3]);
let mut acc = Vec::new();
let mut f: Box<FnMut(&(i32, Vec<i32>))> = Box::new(|(_, mut v)| {
    acc.append(&mut v);
});
f(&a);
