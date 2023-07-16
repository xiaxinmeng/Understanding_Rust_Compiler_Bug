 rust
#![feature(macro_rules)]

macro_rules! each {
    ($e:ident in $y:ident, $b:block) => {{
        loop {
            let $e = match $y.next() {
                Some(e) => e,
                None => break,
            };
            $b;
        }
    }}
}

fn main() {
    let mut x = range(0, 10i);
    each!(y in x, {
        println!("{}", y);
        x.next();
    });
}
