
error: `$stype:ty` may be followed by `functions`, which is not allowed for `ty` fragments
  --> /home/marcel/.cargo/registry/src/github.com-1ecc6299db9ec823/dlib-0.1.1/src/lib.rs:39:11
   |
39 |         $(functions: $(fn $fname: ident($($farg: ty),*) -> $fret:ty),+)|*
   |           ^^^^^^^^^ not allowed after `ty` fragments
   |
   = note: allowed there are: `{`, `[`, `=>`, `,`, `>`, `=`, `:`, `;`, `|`, `as` or `where`
