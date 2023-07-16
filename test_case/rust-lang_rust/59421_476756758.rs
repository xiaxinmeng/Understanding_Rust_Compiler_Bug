
error: expected identifier, found `0suffix`
  --> src/main.rs:12:17
   |
12 |     let s = X { 0suffix: 0, 1: 1, 2: 2 };
   |             -   ^^^^^^^ expected identifier
   |             |
   |             while parsing this struct

error: expected identifier, found `0suffix`
  --> src/main.rs:15:13
   |
15 |         X { 0suffix: _, .. } => {}
   |             ^^^^^^^ expected identifier

error[E0063]: missing field `0` in initializer of `X`
  --> src/main.rs:12:13
   |
12 |     let s = X { 0suffix: 0, 1: 1, 2: 2 };
   |             ^ missing `0`

error[E0027]: pattern does not mention fields `0`, `1`, `2`
  --> src/main.rs:15:9
   |
15 |         X { 0suffix: _, .. } => {}
   |         ^^^^^^^^^^^^^^^^^^^^ missing fields `0`, `1`, `2`
   |
   = note: trying to match a tuple variant with a struct variant pattern
