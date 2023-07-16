rust
use std::collections::HashMap;

trait Foo {
    type Value;
}

struct Holder<T: Foo>(HashMap<String, T::Value>);

fn foo<T: Foo>(hm: HashMap<String, T::Value>) {
    let holder = Holder(hm);
    if let Some(v) = holder.0.get("abc") {
        let v = v.clone();
    }
}
