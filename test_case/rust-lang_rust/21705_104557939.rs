 rust
pub type Callback = extern fn();

extern {
    pub fn something(arg: Callback);
}

extern fn cb() {
    unsafe{ something(cb); }
}
