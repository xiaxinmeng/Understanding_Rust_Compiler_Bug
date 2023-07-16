rs
trait Pointee {
    type Meta: 'static + Copy;
}

struct SomethingStatic<T: 'static>(T);
impl<T> Pointee for SomethingStatic<T> {
    type Meta = fn(T);
}

struct CovariantPtr<T>(T);

fn demonstrate_subtyping(
    x: CovariantPtr<SomethingStatic<for<'a> fn(&'a str)>>,
) -> CovariantPtr<SomethingStatic<fn(&'static str)>> {
    x
}
