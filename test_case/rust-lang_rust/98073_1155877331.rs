plain
     |
2144 |     U8 = 0,
     |     ^^^^^^
     |
     = note: `-D dead-code` implied by `-D warnings`
note: `InvariantSize` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
     |
2141 |   #[derive(Debug, Clone, Copy)]
     |            ^^^^^  ^^^^^ in this derive macro expansion
     |            |
