 rust
fn random_sample<A, T>(mut iter: A) -> Option<T>
    where A: Iterator<T> {
{
    let mut elem = None;
    let mut i = 1f64;
    for new_item in iter {
        if std::rand::random::<f64>() < (1f64/i) {
            elem = Some(new_item);
        }
        i += 1.0;
    }
    elem
}
