
error[E0425]: cannot find value `foo` in this scope
 --> file4.rs:5:9
  |
5 |         foo: Vec::new()
  |         ^^^
  |         |
  |         `self` value is a keyword only available in methods with `self` parameter
  |         help: try: `self.foo`

error[E0658]: type ascription is experimental (see issue #23416)
 --> file4.rs:5:9
  |
5 |         foo: Vec::new()
  |         ^^^^^^^^^^^^^^^
  |
  = help: add #![feature(type_ascription)] to the crate attributes to enable

error: parenthesized type parameters may only be used with a `Fn` trait
 --> file4.rs:5:22
  |
5 |         foo: Vec::new()
  |                      ^^
  |
  = note: #[deny(parenthesized_params_in_types_and_modules)] on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>

error[E0107]: wrong number of type arguments: expected 1, found 0
 --> file4.rs:5:14
  |
5 |         foo: Vec::new()
  |              ^^^^^^^^^^ expected 1 type argument

error: aborting due to 5 previous errors
