plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0080]: evaluation of constant value failed
   --> library/core/src/ffi/c_str.rs:174:9
    |
174 |         const SLICE: &[c_char] = &[0];
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
error[E0080]: evaluation of constant value failed
   --> library/core/src/unicode/printable.rs:76:1
    |
    |
76  | / const SINGLETONS0U: &[(u8, u8)] = &[
77  | |     (0x00, 1),
78  | |     (0x03, 5),
79  | |     (0x05, 6),
...   |
116 | |     (0xff, 9),
117 | | ];
    | |__^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
error[E0080]: evaluation of constant value failed
   --> library/core/src/unicode/printable.rs:119:1
    |
    |
119 | / const SINGLETONS0L: &[u8] = &[
120 | |     0xad, 0x78, 0x79, 0x8b, 0x8d, 0xa2, 0x30, 0x57,
121 | |     0x58, 0x8b, 0x8c, 0x90, 0x1c, 0xdd, 0x0e, 0x0f,
122 | |     0x4b, 0x4c, 0xfb, 0xfc, 0x2e, 0x2f, 0x3f, 0x5c,
...   |
155 | |     0xc9, 0xd0, 0xd1, 0xd8, 0xd9, 0xe7, 0xfe, 0xff,
156 | | ];
    | |__^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
error[E0080]: evaluation of constant value failed
   --> library/core/src/unicode/printable.rs:158:1
    |
    |
158 | / const SINGLETONS1U: &[(u8, u8)] = &[
159 | |     (0x00, 6),
160 | |     (0x01, 1),
161 | |     (0x03, 1),
...   |
200 | |     (0xfb, 1),
201 | | ];
    | |__^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
error[E0080]: evaluation of constant value failed
   --> library/core/src/unicode/printable.rs:203:1
    |
    |
203 | / const SINGLETONS1L: &[u8] = &[
204 | |     0x0c, 0x27, 0x3b, 0x3e, 0x4e, 0x4f, 0x8f, 0x9e,
205 | |     0x9e, 0x9f, 0x7b, 0x8b, 0x93, 0x96, 0xa2, 0xb2,
206 | |     0xba, 0x86, 0xb1, 0x06, 0x07, 0x09, 0x36, 0x3d,
...   |
227 | |     0xb0, 0xc0, 0xd0, 0xae, 0xaf, 0x6e, 0x6f, 0x93,
228 | | ];
    | |__^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
error[E0080]: evaluation of constant value failed
   --> library/core/src/unicode/printable.rs:230:1
    |
    |
230 | / const NORMAL0: &[u8] = &[
232 | |     0x5f, 0x22,
233 | |     0x82, 0xdf, 0x04,
...   |
366 | |     0x0f, 0x0d,
366 | |     0x0f, 0x0d,
367 | | ];
    | |__^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
error[E0080]: evaluation of constant value failed
   --> library/core/src/unicode/printable.rs:369:1
    |
    |
369 | / const NORMAL1: &[u8] = &[
370 | |     0x5e, 0x22,
371 | |     0x7b, 0x05,
...   |
565 | |     0x0a, 0x84, 0x06,
566 | | ];
566 | | ];
    | |__^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
error[E0080]: could not evaluate static initializer
    --> library/core/src/unicode/unicode_data.rs:589:5
     |
     |
589  | /     static LOWERCASE_TABLE: &[(char, [char; 3])] = &[
590  | |         ('A', ['a', '\u{0}', '\u{0}']), ('B', ['b', '\u{0}', '\u{0}']),
591  | |         ('C', ['c', '\u{0}', '\u{0}']), ('D', ['d', '\u{0}', '\u{0}']),
592  | |         ('E', ['e', '\u{0}', '\u{0}']), ('F', ['f', '\u{0}', '\u{0}']),
...    |
1436 | |         ('\u{1e921}', ['\u{1e943}', '\u{0}', '\u{0}']),
1437 | |     ];
     | |______^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
error[E0080]: could not evaluate static initializer
    --> library/core/src/unicode/unicode_data.rs:1439:5
     |
     |
1439 | /     static UPPERCASE_TABLE: &[(char, [char; 3])] = &[
1440 | |         ('a', ['A', '\u{0}', '\u{0}']), ('b', ['B', '\u{0}', '\u{0}']),
1441 | |         ('c', ['C', '\u{0}', '\u{0}']), ('d', ['D', '\u{0}', '\u{0}']),
1442 | |         ('e', ['E', '\u{0}', '\u{0}']), ('f', ['F', '\u{0}', '\u{0}']),
...    |
2373 | |         ('\u{1e943}', ['\u{1e921}', '\u{0}', '\u{0}']),
2374 | |     ];
     | |______^ scalar size mismatch: expected 4 bytes but got 8 bytes instead
For more information about this error, try `rustc --explain E0080`.
error: could not compile `core` due to 9 previous errors
Build completed unsuccessfully in 0:00:29
