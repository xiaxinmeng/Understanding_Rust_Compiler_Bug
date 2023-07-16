 rust
#![feature(type_ascription)]

fn main() {
    match 42: Result<_,_> {
        Err(err) => return Err(From::from(err)),
        Ok(val) => val,
    };
}
