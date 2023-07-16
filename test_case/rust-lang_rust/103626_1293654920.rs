rust
pub trait FromResidual<R = <Self as Try>::Residual> {
    fn from_residual(residual: R) -> Self;
}

trait Try {
    type Residual;
}

fn w<'a, F>()
where
    F: Fn(&'a U),
{
    let b: &dyn FromResidual = &();
}
