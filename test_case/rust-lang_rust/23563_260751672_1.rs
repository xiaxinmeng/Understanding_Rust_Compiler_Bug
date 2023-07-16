 rust
extern crate a;

use a::LolFrom;
use a::LolInto;
use a::LolTo;

struct LocalType<T>(Option<T>);

impl<'a, T> LolFrom<&'a [T]> for LocalType<T> {
    fn from(_: &'a [T]) -> LocalType<T> { LocalType(None) }
}

impl<T> LolInto<LocalType<T>> for LocalType<T> {
    fn convert_into(self) -> LocalType<T> {
        self
    }
}

impl LolTo<LocalType<u8>> for [u8] {
    fn convert_to(&self) -> LocalType<u8> {
        LocalType(None)
    }
}
