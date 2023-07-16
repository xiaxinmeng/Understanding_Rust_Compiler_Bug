 rust
#![feature(default_type_parameter_fallback)]

use Option::*;
use std::borrow::Borrow;

pub enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    pub fn as_ref<U: ?Sized = T>(&self) -> Option<&U> where T: Borrow<U> {
        match *self {
            None => None,
            Some(ref t) => Some(t.borrow()),
        }
    }
}

#[test]
fn test() {
    let opt = Some("foo".to_string());
    let _r1 = opt.as_ref(); // falls back to `Option<&String>`
    let _r2 = opt.as_ref::<str>();
}
