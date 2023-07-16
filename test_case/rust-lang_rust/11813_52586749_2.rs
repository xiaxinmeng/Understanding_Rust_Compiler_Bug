 rust
extern crate mylib;

#[phase(plugin)]
extern crate mylib_plugin;

use mylib::MyTrait;

#[deriving(MyTrait)]
struct Foo;
