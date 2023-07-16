rs
let mut inner = [1usize, 2, 3, 4, 5];
let mut outer = [&mut inner];

for items in &mut outer {
    for item in items {
        *item += 1
    }
}
