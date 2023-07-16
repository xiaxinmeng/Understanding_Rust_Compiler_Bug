rust
// aux-build:alias-reexport.rs
// aux-build:alias-reexport2.rs

#![crate_name = "foo"]

extern crate alias_reexport2;

// @has 'foo/reexport/fn.foo.html'
// @has - '//*[@class="docblock item-decl"]' 'pub fn foo() -> Reexport'
// @has 'foo/reexport/fn.foo2.html'
// @has - '//*[@class="docblock item-decl"]' 'pub fn foo2() -> Result<Reexport, ()>'
// @has 'foo/reexport/type.Reexported.html'
// @has - '//*[@class="docblock item-decl"]' 'pub type Reexported'
#[doc(inline)]
pub use alias_reexport2 as reexport;
