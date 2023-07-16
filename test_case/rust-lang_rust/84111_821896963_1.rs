
error[E0282]: type annotations needed for `HashMap<i32, i32, S>`
 --> src/main.rs:5:15
  |
5 |     let map = HashMap::from_iter(vec![
  |         ---   ^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `S` declared on the struct `HashMap`
  |         |
  |         consider giving `map` the explicit type `HashMap<i32, i32, S>`, where the type parameter `S` is specified
