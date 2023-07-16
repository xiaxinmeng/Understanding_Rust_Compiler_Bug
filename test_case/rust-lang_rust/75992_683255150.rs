
error: reached the type-length limit while instantiating `<std::boxed::Box<std::future::fr..., ()}]>>, ()}]>, ()}]>>>>>::into`
   --> /Users/martin/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/src/libcore/convert/mod.rs:559:5
    |
559 | /     fn into(self) -> U {
560 | |         U::from(self)
561 | |     }
    | |_____^
    |
    = note: consider adding a `#![type_length_limit="1527629"]` attribute to your crate
