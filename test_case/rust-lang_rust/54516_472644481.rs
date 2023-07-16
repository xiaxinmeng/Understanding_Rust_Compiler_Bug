
warning: unnecessary path disambiguator
 --> file2.rs:3:36
  |
3 |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>,());
  |                                    ^^ try removing `::`

error: argument never used
 --> file2.rs:3:59
  |
3 |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>,());
  |              ---- formatting specifier missing            ^^ argument never used

error[E0423]: expected value, found module `std::mem`
 --> file2.rs:3:20
  |
3 |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>,());
  |                    ^^^^^^^^- help: maybe you meant to write a path separator here: `::`
  |                    |
  |                    not a value

error[E0412]: cannot find type `size_of` in this scope
 --> file2.rs:3:29
  |
3 |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>,());
  |                            -^^^^^^^ not found in this scope
  |                            |
  |                            help: maybe you meant to write a path separator here: `::`

error[E0658]: type ascription is experimental (see issue #23416)
 --> file2.rs:3:20
  |
3 |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>,());
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: add #![feature(type_ascription)] to the crate attributes to enable
