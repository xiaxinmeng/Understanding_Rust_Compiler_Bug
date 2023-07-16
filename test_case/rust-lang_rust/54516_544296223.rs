
error: expected one of `!`, `,`, or `::`, found `(`
 --> file8.rs:3:58
  |
3 |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
  |                            -                             ^ expected one of `!`, `,`, or `::` here
  |                            |
  |                            help: maybe write a path separator here: `::`
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
  = note: for more information, see https://github.com/rust-lang/rust/issues/23416
