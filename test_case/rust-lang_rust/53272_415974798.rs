
error: anonymous parameters are deprecated and will be removed in the next edition.
   --> libstd\sys\windows\backtrace\mod.rs:155:20
    |
155 |     fn walk(&self, c::DWORD, c::HANDLE, c::HANDLE, &mut Self::Item, &mut c::CONTEXT) -> c::BOOL;
    |                    ^^^^^^^^ help: Try naming the parameter or explicitly ignoring it: `_: c::DWORD`
    |
    = note: `-D anonymous-parameters` implied by `-D warnings`
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
