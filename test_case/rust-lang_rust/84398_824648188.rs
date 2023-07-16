rust
pub trait Deserialize<'de>: Sized {}
pub trait DeserializeOwned: for<'de> Deserialize<'de> {}

pub trait Extensible {
    type Config;
}

pub trait Installer<C> {
    fn init<B: Extensible<Config = C>>(&mut self) -> ()
    where
        B::Config: DeserializeOwned,
    {
    }
}
