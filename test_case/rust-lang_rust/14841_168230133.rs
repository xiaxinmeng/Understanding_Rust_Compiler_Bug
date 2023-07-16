 rust
// plugin_lib.rs
#![feature(plugin_registrar)]

#[plugin_registrar]
pub fn plugin_registrar(f: fn() -> ()) {
    f()
}
