rust
let x = if condition {
    ....
} else #[cold] {
   ....
};

let y = match condition2 {
    Option(x) => x,
    None => #[cold] 1
};
