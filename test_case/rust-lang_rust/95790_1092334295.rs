plain
---- src/collections/hash/map.rs - collections::hash::map::HashMap<K,V,RandomState>::from (line 1219) stdout ----
error[E0308]: mismatched types
 --> src/collections/hash/map.rs:1223:40
  |
7 | let map2: HashMap<_, _> = [vec!(1, 2), (3, 4)].into();
  |                                        ^^^^^^ expected struct `Vec`, found tuple
  |
  = note: expected type `Vec<{integer}>`
            found tuple `({integer}, {integer})`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
