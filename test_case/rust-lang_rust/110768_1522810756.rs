rust
fn test<Iter, X, Y>(x: X) -> Y
    where
        Iter: Iterator<Item=X>,
        Iter: Iterator<Item=Y>,
{
    x
}
