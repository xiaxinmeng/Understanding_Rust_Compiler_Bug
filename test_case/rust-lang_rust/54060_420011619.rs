rust
let i = vec![1, 2, 3];
let closures = vec![];
for &x in &something {
    closures.push(|| use(i, x)); // ERROR because `x` is out of scope
}
