rust
#![feature(decl_macro)]

mod x {
    macro private() {}

    pub macro public() {
        private!();
    }
}

x::public!();
