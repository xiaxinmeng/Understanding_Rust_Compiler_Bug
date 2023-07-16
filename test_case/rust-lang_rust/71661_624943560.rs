rust
let mut o = map.entry("key").occupied().unwrap();
o.get_mut().set_foo(1);
if o.get().bar() {
    things.push(o.remove())
}
