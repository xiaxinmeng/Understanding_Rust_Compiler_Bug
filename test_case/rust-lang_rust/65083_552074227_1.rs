rust
// aux-build:stability_attribute_generic.rs

extern crate stability_attribute_generic;
use stability_attribute_generic::*;

fn new_foo(f: Foo) {}

fn new_foo_t<T>(f: Foo<T>) {}

fn new_foo_str(f: Foo<&str>) {}

fn main() {}
