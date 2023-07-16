plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error: mismatched closing delimiter: `)`
   --> compiler/rustc_middle/src/ty/util.rs:818:61
    |
818 |     pub fn is_user_visible_dep(self, key: CrateNum) -> bool {
...
...
829 |             || self.extern_crate(key.as_def_id()).map_or(false, |e| e.is_direct()))

error: unexpected closing delimiter: `}`
   --> compiler/rustc_middle/src/ty/util.rs:831:1
    |
    |
829 |             || self.extern_crate(key.as_def_id()).map_or(false, |e| e.is_direct()))
    |                                                                                   - missing open `(` for this delimiter
831 | }
    | ^ unexpected closing delimiter

error: could not compile `rustc_middle` (lib test) due to 2 previous errors
