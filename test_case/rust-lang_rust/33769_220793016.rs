 rust
macro_rules! useassoc {
    ($t:ident, $assoc:ident) => { ($t, $t::$assoc) }
}

trait HasAssoc { type Type; }

#[derive(Debug)]
struct Thing1<T: HasAssoc> {
    thing: useassoc!(T, Type)
}
#[derive(Debug)]
struct Thing2<T: HasAssoc> {
    thing: (T, T::Type)
}
