
error[E0282]: type annotations needed for `impl Future`
  --> $DIR/cannot-infer-async-enabled-impl-trait-bindings.rs:13:20
   |
LL |     let fut = async {
   |         --- consider giving `fut` the explicit type `impl Future`, with the type parameters specified
LL |         make_unit()?;
   |                    ^ cannot infer type
