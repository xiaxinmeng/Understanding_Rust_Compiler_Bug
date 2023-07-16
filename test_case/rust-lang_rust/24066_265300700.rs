`
fn f<V>(_: V) -> String where String: From<V> { String::from("hello") }
