 rust
#![feature(local_prelude)]

mod prelude {
    #[local_prelude]
    mod v1 { /* the libcore or libstd prelude */ }
}
