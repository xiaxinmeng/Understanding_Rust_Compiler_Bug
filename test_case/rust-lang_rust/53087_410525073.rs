Rust
#![feature(existential_type)]
#![feature(const_fn)]
#![feature(untagged_unions)]

const fn transmute<T, U>(t: T) -> U {
    union Transform<TT, UU> {
        t: TT,
        u: UU,
    }
    
    unsafe { Transform { t }.u }
}


existential type Foo: Fn() + Copy;
const BAZR: Foo = transmute(|| {});

fn bar() -> Foo { || {} }

fn main() {
    let x = BAZR();
    
    println!("{:?}", x);
}
