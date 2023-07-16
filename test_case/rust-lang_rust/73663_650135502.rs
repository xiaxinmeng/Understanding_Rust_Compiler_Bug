rust
// test.rs
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct KeyType;
struct Value;

trait IntoCborKey {
    fn into_cbor_key(self) -> KeyType;
}
trait IntoCborValue {
    fn into_cbor_value(self) -> Value;
}

impl IntoCborValue for i32 {
    fn into_cbor_value(self) -> Value {
        Value
    }
}

impl IntoCborKey for i32 {
    fn into_cbor_key(self) -> KeyType {
        KeyType
    }
}

macro_rules! cbor_map {
    ( $( $key:expr => $value:expr, )+ ) => {
        cbor_map! ( $($key => $value),+ )
    };

    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut _map = BTreeMap::new();
            $(
                _map.insert($key.into_cbor_key(), $value.into_cbor_value());
            )*
            _map
        }
    };
}

fn main() {
    let _ = cbor_map! {
        1 => 1,
        2 => 2,
        3 => 3,
        #[cfg(test)]
        4 => 4,
    };
}
