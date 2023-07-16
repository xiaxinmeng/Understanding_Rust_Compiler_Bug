rust
>#![forbid(clippy::expect, clippy::unwrap)]
>
>fn main() {
>  let _arr: [i32; 10] = (0..).map(f).collect().unwrap(); // Error!
>}
>