 rust
fn each<'r, T>(v: &'r [T] ,f: &fn(&'r T) -> bool) -> bool { ... }
impl<'self, T> BaseIter<T> for &'self [T] { 
    fn each<'r>(&'r self, &fn(&'r T) -> bool) -> bool { ... }
}
