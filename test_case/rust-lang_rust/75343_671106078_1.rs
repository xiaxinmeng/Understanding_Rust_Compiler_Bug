
error[E0283]: type annotations needed
 --> src/main.rs:9:15
  |
1 | trait Deserialize<'de> {
  | ---------------------- required by this bound in `Deserialize`
...
9 | impl <'de, T: Deserialize> Deserialize<'de> for Container<T> where T: Deserialize<'de> {
  |               ^^^^^^^^^^^ cannot infer type for type parameter `T`
  |
  = note: cannot satisfy `T: Deserialize<'static>`
help: consider specifying the type arguments in the function call
  |
9 | impl <'de, T: Deserialize::<Self, 'de>> Deserialize<'de> for Container<T> where T: Deserialize<'de> {
  |                          ^^^^^^^^^^^^^
