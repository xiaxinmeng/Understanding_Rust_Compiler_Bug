rust
trait Trait { type Assoc; }

struct Foo<T : Trait>(T::Assoc);

impl Trait for fn(&'static ()) {
    type Assoc = ();
}

fn main() {
    let sendable_future: &dyn Send = &async {
        let s = Foo::<fn(&'static ())>(());
        std::mem::drop(s);
        async{}.await;
    };
}
