rust
#![feature(generators, generator_trait)]

fn main() {
    let _gen = move || {
        {
            let x = [1u8; 1024];
            yield;
            drop(x);
        }
        {
            let x = [1u8; 1024];
            yield;
            drop(x);
        }
        {
            let x = [1u8; 1024];
            yield;
            drop(x);
        }
        {
            let x = [1u8; 1024];
            yield;
            drop(x);
        }
    };
    dbg!(std::mem::size_of_val(&_gen));
}

