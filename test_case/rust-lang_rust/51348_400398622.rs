rust
#![feature(nll)]

pub struct Foo {
    pub bar: Option<String>,
    pub baz: Option<Box<Baz>>,
}

pub enum Baz {
    A(Foo),
    B(Foo, String),
}

fn sadness() {
    use Baz::*;

    let _fail = |mut f: Foo| -> Foo {
        f.bar = match f.bar {
            None => match f.baz {
                Some(ref mut f) => {
                    match (*f).as_mut() {
                        | &mut A(ref mut foo) | &mut B(ref mut foo, _) if foo.bar.is_none() => {
                            foo.bar = None;
                        }
                        _ => {}
                    }
                    None
                }
                None => None,
            },
            Some(x) => Some(x),
        };
        f
    };
}
