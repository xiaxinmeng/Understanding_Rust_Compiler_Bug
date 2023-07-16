text
   Compiling playground v0.0.1 (/playground)
warning: `dyn B` implements `Deref` with supertrait `(dyn A + 'static)` as output
  --> src/lib.rs:17:12
   |
17 |     take_a(b)
   |            ^
   |
   = note: `#[warn(deref_into_dyn_supertrait)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #89460 <https://github.com/rust-lang/rust/issues/89460>

warning: `playground` (lib) generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 1.28s
