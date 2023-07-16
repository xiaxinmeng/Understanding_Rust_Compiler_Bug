
#[cfg(test)]
mod test{
    use super::MyImpl;
    use std::Trait1;
    use std::Trait2;
    use std::Trait3;

    #[test_impl(MyImpl, Trait1)]
    #[test_impl(MyImpl, Trait2)]
    #[test_impl(MyImpl, Trait3)]
}
