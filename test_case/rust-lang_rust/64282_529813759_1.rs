[src/lib.rs:6] &input = TokenStream [
    Ident {
        ident: "fn",
        span: #0 bytes(0..0),
    },
    Ident {
        ident: "hello",
        span: #0 bytes(0..0),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Ident {
                ident: "a",
                span: #0 bytes(0..0),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #0 bytes(0..0),
            },
            Ident {
                ident: "i32",
                span: #0 bytes(0..0),
            },
            Punct {
                ch: ',',
                spacing: Alone,
                span: #0 bytes(0..0),
            },
            Ident {
                ident: "b",
                span: #0 bytes(0..0),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #0 bytes(0..0),
            },
            Ident {
                ident: "i32",
                span: #0 bytes(0..0),
            },
        ],
        span: #0 bytes(0..0),
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [],
        span: #0 bytes(0..0),
    },
]
error[E0658]: The attribute `angery` is currently unknown to the compiler and may have meaning added to it in the future
 --> proc-macro-tests/rename_params.rs:5:10
  |
5 | fn hello(#[angery(true)] a: i32, #[a2] b: i32) {}
  |          ^^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/29642
  = help: add `#![feature(custom_attribute)]` to the crate attributes to enable

error[E0658]: The attribute `a2` is currently unknown to the compiler and may have meaning added to it in the future
 --> proc-macro-tests/rename_params.rs:5:34
  |
5 | fn hello(#[angery(true)] a: i32, #[a2] b: i32) {}
  |                                  ^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/29642
  = help: add `#![feature(custom_attribute)]` to the crate attributes to enable

error: aborting due to 2 previous errors