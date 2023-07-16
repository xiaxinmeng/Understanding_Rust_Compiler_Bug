
error[E0597]: `data` does not live long enough
  --> src/main.rs:9:19
   |
9  |         for d in &data {
   |                   ^^^^ does not live long enough
...
12 |     }
   |     - borrowed value only lives until here
   |
note: borrowed value must be valid for the lifetime 'a as defined on the function body at 5:1...
  --> src/main.rs:5:1
   |
5  | / fn gen<'a, T>(data: Vec<&'a T>)
6  | |     -> impl Generator<Yield = &&T, Return = ()> + 'a
7  | | {
8  | |     move || {
...  |
12 | |     }
13 | | }
   | |_^

error: aborting due to previous error

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
