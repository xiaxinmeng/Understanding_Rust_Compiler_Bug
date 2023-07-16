 rust
let s = "ë";  // two combined code points
s.byte_len();  // length of the string in bytes (3)
s.char_len();  // length of the string in code points (2)
s.rune_len();  // length of the string in grapheme clusters (1)
