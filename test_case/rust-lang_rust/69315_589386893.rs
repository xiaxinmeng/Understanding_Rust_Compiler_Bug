rust
pub mod wasm {
    #[the_macro::the_macro]
    extern "C" {
        pub type Memory;
    }
}

fn main() {
    let _: wasm::Memory;
}
