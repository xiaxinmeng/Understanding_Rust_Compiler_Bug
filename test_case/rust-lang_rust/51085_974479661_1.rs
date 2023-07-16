rust
#![feature(exhaustive_patterns)]
#![feature(never_type)]

pub enum Value<T, K, U> {
    Known(T, K),
    Unknown(U),
}

fn test(value: Value<bool, (), !>) -> bool {
    let Value::Known(value, _) = value;
    value
}
