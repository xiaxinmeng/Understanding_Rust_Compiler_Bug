rust
trait RefMul: Sized + Mul + for<'a> Mul<&'a Self> 
where
    // We refer to this as the "ref bound" below
    for<'a> &'a Self: Mul<&'a Self, Output=Self>
{}

fn do_mul<T: RefMul + Clone>(a: T, b: T) {
    // Works without the ref bound! T * T
    a.clone() * b.clone();
    // Works without the ref bound! T * &T
    a.clone() * &b;
    // But sadly we can't do anything where we need to use &T
    // on the left hand side like the two below expressions :(
    &a * b.clone();
    &a * &b;
}
