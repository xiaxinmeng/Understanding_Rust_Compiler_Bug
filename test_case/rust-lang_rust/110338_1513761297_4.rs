rust
use std::future::Future;

trait Trait { type Assoc; }

struct Foo<T : Trait>(T::Assoc);

impl Trait for fn(&'static ()) {
    type Assoc = ();
}

fn main() {
    let f: &dyn Future<Output = ()> = &async {
        let s = Foo::<fn(&'static ())>(());
        async{}.await;
    };
}
