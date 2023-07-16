rust
 error: `write` is both a function and a macro
   --> library/core/src/ptr/mod.rs:132:13
    |
132 | /// again. [`write`] can be used to overwrite data without causing it to be
    |             ^^^^^^^ ambiguous link
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
help: to link to the function, add parentheses
    |
132 | /// again. [`write()`] can be used to overwrite data without causing it to be
    |             ^^^^^^^^^
help: to link to the macro, add an exclamation mark
    |
132 | /// again. [`write!`] can be used to overwrite data without causing it to be
    |             ^^^^^^^^
