rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

macro_rules! bound_size {
    () => {1usize}
}

struct BracesAreNOTcheckedBeforeICE<const U: usize = bound_size!() as usize>
where [(); U]:;
