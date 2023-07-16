
warning: `#[derive]` can't be used on a `#[repr(packed)]` struct that does not derive Copy (error E0133)
   --> src/v4l2.rs:460:58
    |
460 | #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    |                                                          ^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(safe_packed_borrows)]` on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
    = note: this warning originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

