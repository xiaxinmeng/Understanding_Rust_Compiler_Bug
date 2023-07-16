text
Compiling playground v0.0.1 (/playground)
error[E0106]: missing lifetime specifier
 --> src/lib.rs:3:34
  |
3 | fn func1<'a>(_arg: &'a Thing) -> &() { unimplemented!() }
  |                                  ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say which one of `_arg`'s 2 lifetimes it is borrowed from

error[E0106]: missing lifetime specifier
 --> src/lib.rs:4:35
  |
4 | fn func2<'a>(_arg: &Thing<'a>) -> &() { unimplemented!() }
  |                                   ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say which one of `_arg`'s 2 lifetimes it is borrowed from

error: aborting due to 2 previous errors
