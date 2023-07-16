plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0107]: this associated function takes 0 type arguments but 1 type argument was supplied
   --> src/librustdoc/html/render/mod.rs:289:35
    |
289 |         (&self.ty.name, self.kind.into::<ItemType>()).serialize(serializer)
    |                                   ^^^^------------ help: remove these generics
    |                                   expected 0 type arguments
    |
    |
note: associated function defined here, with 0 type parameters
    |
    |
275 |     fn into(self) -> T;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
