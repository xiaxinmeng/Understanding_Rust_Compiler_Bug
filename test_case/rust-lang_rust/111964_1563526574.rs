
> 106 | pub struct HashSet<T, S = RandomState> {
> 
> error: negative impls should be always applicable
>    --> compiler/rustc_data_structures/src/stable_hasher.rs:615:15
>     |
>     |
> 615 | impl<V, HCX> !HashStable<HCX> for std::collections::HashSet<V> {}
>     |               ^^^^^^^^^^^^^^^ negative impls
> 