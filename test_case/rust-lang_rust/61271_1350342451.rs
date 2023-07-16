
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/lib.rs:5:5
  |
1 |   fn permute<'a, T, I>(sides: &'a [T]) -> I
  |                     -                     -
  |                     |                     |
  |                     |                     expected `I` because of return type
  |                     this type parameter   help: consider using an impl return type: `impl Iterator<Item = Vec<&'a T>>`
...
5 | /     sides.iter().enumerate().map(|(i, _)| {
6 | |         let window = sides.iter().cycle().skip(i);
7 | |         window.take(sides.len()).collect()
8 | |     })
  | |______^ expected type parameter `I`, found struct `Map`
  |
  = note: expected type parameter `I`
                     found struct `Map<Enumerate<std::slice::Iter<'_, T>>, [closure@src/lib.rs:5:34: 5:42]>`
