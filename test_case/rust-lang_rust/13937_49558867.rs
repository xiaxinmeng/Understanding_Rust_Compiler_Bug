 rust
macro_rules! early_return {
    ($inp:expr $sp:ident) => { // invoke it like `(input_5 SpecialE)`
        match $inp {
            $sp(x) => { return x; }
            _ => {}
        }
    };
}
