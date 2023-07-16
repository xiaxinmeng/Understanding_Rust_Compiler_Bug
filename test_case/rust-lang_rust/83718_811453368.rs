rust
pub trait Load {
    fn load() {}
}

impl<P> Load for P {
    fn load() {}
}

pub struct Wrapper {}
