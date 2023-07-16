rust
macro foo($mod_name:ident, $bar:ident) {
    pub mod $mod_name {
        pub const $bar: u32 = 1;
    }
}

foo!(FooMod, BAR);
