rust
use std::fmt;

trait DebugExt: fmt::Debug {}

pub struct Inner<T>(T);

impl fmt::Debug for Inner<()> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct Outer<T>(Inner<T>);
