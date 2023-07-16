 rust
// A procedural macro can expand to an ExternCrate AST node.
extern_crate! extra;
// Similar here, though attributes may not map as well to the AST.
#[extern_crate(extra)];
// These should still work (do we allow visibility on extern mod?):
#[cfg(foo="bar")]
pub extern_crate! foobar;
