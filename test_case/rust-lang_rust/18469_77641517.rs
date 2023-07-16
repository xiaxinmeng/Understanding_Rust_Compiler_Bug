 Rust
fn unify<T>(a: T, _: T) -> T { a }
trait Confusing { fn get() -> Self; }
impl Confusing for Box<[u32]> { fn get() -> Box<[u32]> { Box::new([0]) } }
impl Confusing for Box<[u32; 3]> { fn get() -> Box<[u32; 3]> { Box::new([7,8,9]) } }

#[cfg(not(type_error))]
fn f(x: bool) -> u32 {
    let a = Confusing::get();
    let v: Box<[u32]> = Box::new([1,2,3]);
    let s = match x {
        true => unify(a, v),
        false => unify(a, Box::new([1,2,3])),
    };
    s[0]
}

#[cfg(type_error)]
fn f(x: bool) -> u32 {
    let a = Confusing::get();
    let v: Box<[u32]> = Box::new([1,2,3]);
    let s = match x {
        false => unify(a, Box::new([1,2,3])),
        true => unify(a, v), //~ ERROR mismatched types
                             //| expected `Box<[u32; 3]>`
                             //|    found `Box<[u32]>`
    };
    s[0]
}

fn main() { println!("{}", f(false)); }
