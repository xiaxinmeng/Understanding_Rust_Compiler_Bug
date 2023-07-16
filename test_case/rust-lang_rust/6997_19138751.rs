 rust
// for loop sugar
for xs.take_while(|x| *x < 5).zip(ys) |(x, y)| { return x + y; }

// equivalent low-calorie code, already compiling today
let mut it = xs.take_while(|x| *x < 5).zip(ys);
loop {
    match it.next() {
        Some((x, y)) => return x + y, // note that this will always return successfully
        None => break
    }
}
