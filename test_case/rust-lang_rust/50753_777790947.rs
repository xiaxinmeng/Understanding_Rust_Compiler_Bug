
trait Xyz {}
struct S;
struct T;
impl Xyz for S {}
impl Xyz for T {}

fn foo_no_never() {
    let mut x /* : Option<S> */ = None;
    let mut first_iter = false;
    loop {
        if !first_iter {
            let y: Box<dyn Xyz>
                = /* Box<$0> is coerced to Box<Xyz> here */ Box::new(x.unwrap());
        }

        x = Some(S);
        first_iter = true;
    }

    let mut y : Option<S> = None;
    // assert types are equal
    std::mem::swap(&mut x, &mut y);
}
