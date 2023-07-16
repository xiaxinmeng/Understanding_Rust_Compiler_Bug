rust
% cat target/debug/build/rust_ice-5cc359b860432322/out/codegen.rs
static KEYWORDS: phf::Map<&'static str, Handler> = ::phf::Map {
    key: 1897749892740154578,
    disps: ::phf::Slice::Static(&[
        (0, 0),
    ]),
    entries: ::phf::Slice::Static(&[
        ("help", hello as fn(_, _)),
    ]),
};
