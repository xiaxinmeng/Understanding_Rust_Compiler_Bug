rust
trait B
where
    Self: A,
    <Self as A>::T: B,
{}
