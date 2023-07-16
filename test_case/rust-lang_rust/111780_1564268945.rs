plain
   Compiling basic-toml v0.1.2
   Compiling askama_derive v0.12.1
    Checking askama v0.12.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0408]: variable `ToolMod` is not bound in all patterns
    |
    |
152 |             ToolMod | NonMacroAttr(..) | Err => Result::Err(()),
    |             -------   ^^^^^^^^^^^^^^^^   ^^^ pattern doesn't bind `ToolMod`
    |             |         |
    |             |         pattern doesn't bind `ToolMod`
    |
    |
help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `path::to::ModOrType::ToolMod`
    |
    |
152 |             ToolMod | NonMacroAttr(..) | Err => Result::Err(()),

For more information about this error, try `rustc --explain E0408`.
error: could not compile `rustdoc` (lib) due to previous error
Build completed unsuccessfully in 0:01:11
