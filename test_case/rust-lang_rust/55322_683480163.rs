
error: `$p:path` is followed by `(`, which is not allowed for `path` fragments
 --> src/lib.rs:2:14
  |
2 |     ($p:path ()) => {};
  |              ^ not allowed after `path` fragments
  |
  = note: allowed there are: `{`, `[`, `=>`, `,`, `>`, `=`, `:`, `;`, `|`, `as` or `where`
