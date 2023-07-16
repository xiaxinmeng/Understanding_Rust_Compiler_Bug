
> cargo build --all-features
   Compiling winapi v0.3.4 (file:///C:/Users/Peter/Code/winapi-rs)
warning: #[derive] can't be used on a non-Copy #[repr(packed)] struct (error E0133)
   --> src\macros.rs:354:54
    |
354 |           STRUCT!{#[cfg_attr(feature = "debug", derive(Debug))] $($rest)*}
    |                                                        ^^^^^
    |
   ::: src\um\wingdi.rs:599:1
    |
599 | / STRUCT!{ #[debug] #[repr(packed)] struct BITMAPFILEHEADER {
600 | |     bfType: WORD,
601 | |     bfSize: DWORD,
602 | |     bfReserved1: WORD,
603 | |     bfReserved2: WORD,
604 | |     bfOffBits: DWORD,
605 | | }}
    | |__- in this macro invocation
    |
    = note: #[warn(safe_packed_borrows)] on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>

    Finished dev [unoptimized + debuginfo] target(s) in 33.10s
