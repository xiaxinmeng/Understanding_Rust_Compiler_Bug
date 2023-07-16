
  --> library/core/src/mem/manually_drop.rs:59:25
   |
59 | /// [`MaybeUninit<T>`]: MaybeUninit
   |                         ^^^^^^^^^^^ cannot be resolved, ignoring
   |
   = note: `-D intra-doc-link-resolution-failure` implied by `-D warnings`
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: `[mem::zeroed]` cannot be resolved, ignoring it.
  --> library/core/src/mem/manually_drop.rs:10:11
   |
10 | /// with [`mem::zeroed`] is undefined behavior.
   |           ^^^^^^^^^^^^^ cannot be resolved, ignoring
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: `[MaybeUninit]` cannot be resolved, ignoring it.
  --> library/core/src/mem/manually_drop.rs:59:25
   |
59 | /// [`MaybeUninit<T>`]: MaybeUninit
   |                         ^^^^^^^^^^^ cannot be resolved, ignoring
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
``

</details>

Let me know if you have trouble fixing them.