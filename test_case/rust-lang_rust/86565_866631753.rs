rs
let guess: u32 = guess.trim().parse().expect("Please type a number!");
// ^ After this line, the previous variable with type `String` is now inaccessible.
...
io::stdin().read_line(&mut guess).expect("Failed to read line");
// ^ read_line expects `&mut String`, while you are passing a `&mut u32`.
let guess: u32 = guess.trim().parse().expect("Please type a number!");
// ^ There is no `.trim()` for `u32`.
