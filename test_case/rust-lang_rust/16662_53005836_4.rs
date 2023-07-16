 rust
struct Pieces<'a> {
    args: &'a [Argument],
    strings: [&'a str]
}

static __STATIC_PIECES: Pieces = // ...
let __args = Arguments::new(__STATIC_PIECES, __args_vec);
// ...
