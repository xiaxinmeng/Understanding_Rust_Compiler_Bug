rust
let x: Option<T> = Some(_);
x.0 = makeT(); // OK because we "know" it's `Some`.
