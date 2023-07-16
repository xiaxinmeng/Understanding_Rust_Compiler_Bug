 rust
enum Chain<'a:'b, 'b> {
    Base(&'a i32),
    Rec(&'a i32, &'b Chain<'a,'b>)
}

fn call_with_rec<'a, 'b, F>(c: Chain<'a, 'b>,
                         x: &'a i32,
                         f: F) where F: for<'c> Fn(Chain<'a, 'c>) {
    f(Chain::Rec(x, &c))
}

fn main() {
    let x = 0;
    let y = 0;

    call_with_rec(Chain::Base(&x), &y, |mut inner| {
        loop {
            match inner {
                Chain::Base(&x) => {
                    println!("Base({})", x);
                    break;
                },
                Chain::Rec(&x, &p) => {
                    println!("Rec({})", x);
                    inner = p;
                }
            }
        }
    });
}
