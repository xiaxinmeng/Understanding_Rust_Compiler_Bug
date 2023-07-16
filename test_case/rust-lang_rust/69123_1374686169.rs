
Compiling playground v0.0.1 (/playground)
error[[E0282]](https://doc.rust-lang.org/stable/error-index.html#E0282): type annotations needed
 --> src/main.rs:4:14
  |
4 |     let map: HashMap<i32, i32, _> = HashMap::from_iter(vec![(1, 1), (2, 2)].into_iter());
  |              ^^^^^^^^^^^^^^^^^^^^ cannot infer type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `playground` due to previous error
