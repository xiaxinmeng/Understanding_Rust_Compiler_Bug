rs
let mut inner = vec![1, 2, 3, 4, 5];
let mut outer = vec![&mut inner];

for items in &mut outer {
    for item in items {
        *item += 1
    }
}
