rust
#![feature(generators, generator_trait)]

fn main() {
    let _gen = || {
        {
            let x = [1u8; 1024];
            yield;
            yield;
            drop(x);
        }
        {
            let x = [1u8; 1024];
            yield;
            yield;
            drop(x);
        }
        {
            let x = [1u8; 1024];
            yield;
            yield;
            drop(x);
        }
        {
            let x = [1u8; 1024];
            yield;
            yield;
            drop(x);
        }

    };
    dbg!(std::mem::size_of_val(&_gen));
}

