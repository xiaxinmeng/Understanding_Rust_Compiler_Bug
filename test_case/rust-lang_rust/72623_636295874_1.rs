rust
mod extra_layer {
    mod foo { // Now `pub mod foo` or `mod foo` matter for `use` potential.
        pub struct Bar;
    }
}

fn main() {
    let x = Bar;
}
