
warning: `[Foo::bar]` cannot be resolved, ignoring it.
 --> duplicate-err.rs:1:6
  |
1 | /// [Foo::bar], [Foo::bar]
  |      ^^^^^^^^ cannot be resolved, ignoring
  |
  = note: `#[warn(intra_doc_link_resolution_failure)]` on by default
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: 1 warning emitted
