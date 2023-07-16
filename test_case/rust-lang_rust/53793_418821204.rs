compile_fail,E0491
trait SomeTrait<'a> {
    type Output;
}

impl<'a, T> SomeTrait<'a> for T {
    type Output = &'a T; // compile error E0491
}
