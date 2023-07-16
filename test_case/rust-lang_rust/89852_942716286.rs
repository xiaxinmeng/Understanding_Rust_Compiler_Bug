
Module {
    is_crate: true,
    items: [
        Id("0:4"),   // pub use crate::repro as repro2;
        Id("0:3"),   // macro_rules! repro
        Id("0:3"),   // Same as above, but shouldn't be here !
    ],
}
