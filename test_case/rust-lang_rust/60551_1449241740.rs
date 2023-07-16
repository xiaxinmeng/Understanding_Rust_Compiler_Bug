rs
#![feature(generic_const_exprs)]

fn _internal_impl<const FLAG: bool>() -> &'static str {
    if FLAG {
        "run for accuracy"
    } else {
        "run for speed"
    }
}

trait Operation {
    const FLAG: bool;
    fn run() -> &'static str where [(); Self::FLAG as usize]: {
        // default implementation
        _internal_impl::<{ Self::FLAG }>()
    }
}
