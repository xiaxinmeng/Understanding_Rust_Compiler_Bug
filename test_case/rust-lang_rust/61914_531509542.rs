
error: expected identifier, found `3`
 --> src/main.rs:2:21
  |
2 |     let x = async { 3 };
  |             -----   ^ expected identifier
  |             |
  |             while parsing this struct

error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `move`
 --> src/main.rs:3:19
  |
3 |     let y = async move { 3 };
  |                   ^^^^ expected one of 7 possible tokens here

error[E0422]: cannot find struct, variant or union type `async` in this scope
 --> src/main.rs:2:13
  |
2 |     let x = async { 3 };
  |             ^^^^^ not found in this scope

error: aborting due to 3 previous errors
