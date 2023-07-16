rust
let xs = [13, 1, 5, 2, 3, 1, 21, 8];
let [a, b, c @ ..] = xs;
println!(r"first element: {a}
second element: {b}
rest: {c:?}", a = a, b = b, c = c);
