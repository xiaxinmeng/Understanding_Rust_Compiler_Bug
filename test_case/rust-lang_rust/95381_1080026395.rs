
error[[E0597]](https://doc.rust-lang.org/nightly/error-index.html#E0597): `leaf` does not live long enough
  [--> src/main.rs:28:22
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)   |
28 |     if let Some(x) = leaf.parent.borrow().upgrade() {
   |                      ^^^^----------------
   |                      |
   |                      borrowed value does not live long enough
   |                      a temporary with access to the borrow is created here ...
...
31 | }
   | -
   | |
   | `leaf` dropped here while still borrowed
   | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `Ref<'_, std::rc::Weak<Node>>`
   |
help: consider adding semicolon after the expression so its temporaries are dropped sooner, before the local variables declared by the block are dropped
   |
30 |     };
   |      +
