rust
#![feature(nll)]

fn produce<'a>()
{
    move || {
        let x: &'a () = &();
    };
}


fn main() {}
