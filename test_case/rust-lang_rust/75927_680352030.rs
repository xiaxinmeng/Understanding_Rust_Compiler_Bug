
 error: `[format]` cannot be resolved, ignoring it.
 --> library/core/src/macros/mod.rs:1:1
  |
1 | #[doc(include = "panic.md")]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D intra-doc-link-resolution-failure` implied by `-D warnings`
  = note: the link appears in this line:
          
          [`format!`] syntax for building a string.
           ^^^^^^^^^
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: `[format]` cannot be resolved, ignoring it.
   --> library/core/src/macros/mod.rs:729:39
    |
729 |     /// All other formatting macros ([`format!`], [`write!`], [`println!`], etc) are
    |                                       ^^^^^^^^^ cannot be resolved, ignoring
    |
    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

 error: `[println]` cannot be resolved, ignoring it.
   --> library/core/src/macros/mod.rs:729:64
    |
729 |     /// All other formatting macros ([`format!`], [`write!`], [`println!`], etc) are
    |                                                                ^^^^^^^^^^ cannot be resolved, ignoring
    |
    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: aborting due to 3 previous errors
