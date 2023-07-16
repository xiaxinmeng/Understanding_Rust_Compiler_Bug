
fn small<'a>(x: &'a mut ()) -> &'a mut () {
    let mut x = Some(x);
    let f = || {
        let x = x.take().unwrap();
        x
    };
    f()
}

fn main() { }
