rust
use self::Chain::{Base, Rec};

#[derive(Copy, Clone)]
enum Chain<'a:'b, 'b> {
    Base(&'a i32),
    Rec(&'a i32, &'b Chain<'a,'b>)
}

fn call_with_rec<'a, 'b, F: for <'c> Fn(Chain<'a, 'c>)>(c: Chain<'a, 'b>,
                         x: &'a i32,
                         f: F) {
    f(Rec(x, &c))
}

fn helper<'a, 'b>(mut inner: Chain<'a, 'b>) {
    loop {
        match inner {
            Base(&x) => {
                println!("Base({})", x);
                break;
            },
            Rec(&x, &p) => {
                println!("Rec({})", x);
                inner = p;
            }
        }
    }
}

fn main() {
    let x = 0i32;
    let y = 0i32;

    call_with_rec(Base(&x), &y, helper);

    call_with_rec(Base(&x), &y, |mut inner| {
       loop {
            match inner {
                Base(&x) => {
                    println!("Base({})", x);
                    break;
                },
                Rec(&x, &p) => {
                    println!("Rec({})", x);
                    inner = p;
                }
            }
        }
    })
}
