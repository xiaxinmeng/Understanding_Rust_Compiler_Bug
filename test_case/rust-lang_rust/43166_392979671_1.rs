
error[E0618]: expected function, found `Model`
 --> src/main.rs:9:13
  |
3 | fn model(model: Model) -> Model {
  | ------------------------------- this function shadowed by a local binding
...
8 |     let mut model = Model;
  |         --------- `Model` defined here and shadows a function named `model`
9 |     model = model(model);
  |             ^^^^^^^^^^^^ not a function
