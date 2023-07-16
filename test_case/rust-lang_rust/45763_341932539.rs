
error: no rules expected the token `derives`
  --> src/librustc/hir/def_id.rs:89:1
   |
89 | / newtype_index!(DefIndex
90 | |     {
91 | |         DEBUG_FORMAT custom
92 | |
...  |
98 | |         const CRATE_DEF_INDEX = 0,
99 | |     });
   | |_______^
   |
   = note: this error originates in a macro outside of the current crate

error: aborting due to previous error

error: Could not compile `rustc`.
