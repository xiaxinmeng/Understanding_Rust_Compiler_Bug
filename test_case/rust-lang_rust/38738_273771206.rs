rust
pub trait BaseHello: Base
    where Self::F: Hello,
          Self::B: Hello
{
}
