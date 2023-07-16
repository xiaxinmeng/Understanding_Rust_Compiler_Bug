
error[E0308]: mismatched types
  --> src/lib.rs:10:14
   |
10 |         Self(bytes)
   |              ^^^^^ expected `{(N + 7) / 8}`, found `{(N + 7) / 8}`
   |
   = note: expected struct `ConstBytes<{(N + 7) / 8}>`
              found struct `ConstBytes<{(N + 7) / 8}>`
