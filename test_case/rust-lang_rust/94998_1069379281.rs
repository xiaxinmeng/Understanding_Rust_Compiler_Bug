rust
pub struct Broken {
    _broken: Box<dyn Fn(&()) -> Box<dyn ToString + '_>>,
}

impl From<Broken> for ()
{
    fn from(_: Broken) -> Self {}
}
