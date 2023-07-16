
error: reached the type-length limit while instantiating `<T as Foo><(&(&(&(&(&(&(&(&(&(&(&(&(&(&(&(&(&(&(&(), &()), &(&()...`
  --> C:\bot\slave\auto-win-msvc-64-opt-rustbuild\build\src/test\ui\issue-37311-type-length-limit\issue-37311.rs:23:5
   |
23 |       fn recurse(&self) {
   |  _____^ starting here...
24 | |         (self, self).recurse();
25 | |     }
   | |_____^ ...ending here
   |
   = note: consider adding a `#![type_length_limit="2097152"]` attribute to your crate

error: aborting due to previous error
