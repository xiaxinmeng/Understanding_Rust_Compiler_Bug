rust
fn stringify<T, M1>(m: M1) -> <M1 as Monad>::Plug<String>
where
    T: core::fmt::Display,
    M1: Monad<Unplug = T>,
{
    m.bind(|x| Some(format!("{}", x)))
}
