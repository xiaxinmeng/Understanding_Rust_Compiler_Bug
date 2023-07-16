
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
  --> $DIR/outlives_bounds2.rs:15:26
   |
LL | type Converter<'a, 'b> = impl ProofForConversion<'a, 'b>;
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined here...
  --> $DIR/outlives_bounds2.rs:15:16
   |
LL | type Converter<'a, 'b> = impl ProofForConversion<'a, 'b>;
   |                ^^
note: ...but the lifetime must also be valid for the lifetime `'b` as defined here...
  --> $DIR/outlives_bounds2.rs:15:20
   |
LL | type Converter<'a, 'b> = impl ProofForConversion<'a, 'b>;
   |                    ^^
note: ...so that the types are compatible
  --> $DIR/outlives_bounds2.rs:15:26
   |
LL | type Converter<'a, 'b> = impl ProofForConversion<'a, 'b>;
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `ProofForConversion<'a, 'b>`
              found `ProofForConversion<'_, '_>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.
