
error: an `#[unstable]` annotation here has no effect
    --> library/core/src/ptr/mod.rs:1846:9
     |
1832 | / macro_rules! maybe_fnptr_doc {
1833 | |     (@ #[$meta:meta] $item:item) => {
1834 | |         #[doc(hidden)]
1835 | |         #[$meta]
...    |
1846 | |         #[$meta]
     | |         ^^^^^^^^
1847 | |         $item
1848 | |     };
1849 | | }
     | |_- in this expansion of `maybe_fnptr_doc!` (#3)
...
1857 | / macro_rules! fnptr_impls_safety_abi {
1858 | |     (#[$meta:meta] $FnTy: ty, $($Arg: ident),*) => {
1859 | |         maybe_fnptr_doc! {
1860 | |             $($Arg)* @
...    |
1918 | /         maybe_fnptr_doc! {
1919 |               $($Arg)* @
1920 |               #[$meta]
1921 |               impl<Ret, $($Arg),*> fmt::Debug for $FnTy {
...
1925 |               }
1926 | |         }
     | |_________- in this macro invocation (#3)
1927 | |     }
1928 | | }
     | |_- in this expansion of `fnptr_impls_safety_abi!` (#2)
1929 |
1930 | / macro_rules! fnptr_impls_args {
1931 | |     ($($Arg: ident),+) => {
1932 | |         fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
1933 | |         fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
...    |
1936 | |         fnptr_impls_safety_abi! { #[unstable(feature = "c_unwind", issue = "74990")] extern "C-unwind" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
     | |         -------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation (#2)
...    |
1951 | |     };
1952 | | }
     | |_- in this expansion of `fnptr_impls_args!` (#1)
...
1966 |   fnptr_impls_args! { A, B, C, D, E, F, G, H, I, J, K, L }
     |   -------------------------------------------------------- in this macro invocation (#1)
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information
