rust
trait X {
    type Item;
}

trait Y {
    type Item: X;
}

impl<T: Y> X for T {
    type Item = <Self::Item as X>::Item;
}

// error[E0275]: overflow evaluating the requirement `<T as X>::Item`
//  --> src/main.rs:9:12
//   |
// 9 | impl<T: Y> X for T {
//   |            ^       ^
