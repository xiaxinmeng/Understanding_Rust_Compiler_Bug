
warning: type annotations needed
  --> src/lib.rs:26:38
   |
26 |                 ptr::write(raw.ptr().offset(i as isize), T::default());
   |                                      ^^^^^^
   |
   = note: `#[warn(tyvar_behind_raw_pointer)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #46906 <https://github.com/rust-lang/rust/issues/46906>
