plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
+++ <stderr output>
+error: `box_syntax` has been removed
+  --> $DIR/regions-lifetime-nonfree-late-bound.rs:LL:CC
+   |
+LL |         test(Some(box |_f: Box<dyn for<'a> FnMut(&'a isize)>| {}));
+   |
+help: use `Box::new()` instead
+   |
+   |
+LL |         test(Some(Box::new(|_f: Box<dyn for<'a> FnMut(&'a isize)>| {})));
+
+error[E0308]: mismatched types
+  --> $DIR/regions-lifetime-nonfree-late-bound.rs:LL:CC
+   |
+   |
+LL |         test(Some(box |_f: Box<dyn for<'a> FnMut(&'a isize)>| {}));
+   |              ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<_>`, found closure
+   |              arguments to this enum variant are incorrect
+   |
+   = note: expected struct `std::boxed::Box<_>`
+   = note: expected struct `std::boxed::Box<_>`
+             found closure `[closure@$DIR/regions-lifetime-nonfree-late-bound.rs:LL:CC]`
+   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
+help: the type constructed contains `[closure@$DIR/regions-lifetime-nonfree-late-bound.rs:LL:CC]` due to the type of the argument passed
+   |
+   |
+LL |         test(Some(box |_f: Box<dyn for<'a> FnMut(&'a isize)>| {}));
+   |                   |
+   |                   |
+   |                   this argument influences the type of `Some`
+  --> RUSTLIB/core/src/option.rs:LL:CC
+   |
+LL |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
+   |     ^^^^
+   |     ^^^^
+help: store this in the heap by calling `Box::new`
+   |
+LL |         test(Some(Box::new(box |_f: Box<dyn for<'a> FnMut(&'a isize)>| {})));
+
+error: aborting due to 2 previous errors
+
+For more information about this error, try `rustc --explain E0308`.
---
full stderr:
error: `box_syntax` has been removed
  --> tests/pass/regions-lifetime-nonfree-late-bound.rs:22:19
   |
LL |         test(Some(box |_f: Box<dyn for<'a> FnMut(&'a isize)>| {}));
   |
help: use `Box::new()` instead
   |
   |
LL |         test(Some(Box::new(|_f: Box<dyn for<'a> FnMut(&'a isize)>| {})));

error[E0308]: mismatched types
  --> tests/pass/regions-lifetime-nonfree-late-bound.rs:22:19
   |
   |
LL |         test(Some(box |_f: Box<dyn for<'a> FnMut(&'a isize)>| {}));
   |              ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<_>`, found closure
   |              arguments to this enum variant are incorrect
   |
   = note: expected struct `std::boxed::Box<_>`
             found closure `[closure@tests/pass/regions-lifetime-nonfree-late-bound.rs:22:23: 22:62]`
             found closure `[closure@tests/pass/regions-lifetime-nonfree-late-bound.rs:22:23: 22:62]`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: the type constructed contains `[closure@tests/pass/regions-lifetime-nonfree-late-bound.rs:22:23: 22:62]` due to the type of the argument passed
   |
   |
LL |         test(Some(box |_f: Box<dyn for<'a> FnMut(&'a isize)>| {}));
   |                   |
   |                   |
   |                   this argument influences the type of `Some`
  --> /checkout/library/core/src/option.rs:570:5
   |
LL |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
   |     ^^^^
   |     ^^^^
help: store this in the heap by calling `Box::new`
   |
LL |         test(Some(Box::new(box |_f: Box<dyn for<'a> FnMut(&'a isize)>| {})));

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
