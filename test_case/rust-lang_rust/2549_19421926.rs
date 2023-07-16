
fn chain<T>(x: Option<T>, blk: fn(T) -> Option<T>) -> Option<T> {
    match x { Some(x2) => blk(x2), None => None }
}
