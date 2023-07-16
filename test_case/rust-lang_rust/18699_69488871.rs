 rust
#![feature(plugin_registrar)]
#![crate_type="dylib"]

#[macro_use] extern crate log;
extern crate rustc;

use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
  info!("plugin registrar called");
}
