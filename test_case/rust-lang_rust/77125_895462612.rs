rust
> struct TypeId {
>     hash: u64,
>     // `str` can be replaced with a length-prefixed string to keep `TypeId`'s size down
>     v0_mangled_type: &'static str,
> }
> impl PartialEq for TypeId {
>     fn eq(&self, other: &TypeId) -> bool {
>         self.hash == other.hash // fast reject (!=)
>         && (
>             self.v0_mangled_type as *const str == other.v0_mangled_type as *const str // fast accept (==)
>             || self.v0_mangled_type == other.v0_mangled_type
>         )
>     }
> }
> 