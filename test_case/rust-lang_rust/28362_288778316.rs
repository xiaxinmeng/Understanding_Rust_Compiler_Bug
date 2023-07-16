
rustc 1.16.0 (30cf806ef 2017-03-10)
error: expected one of `!`, `(`, `::`, `;`, `<`, or `}`, found `.`
 --> <anon>:4:11
  |
4 |     s as S.f;
  |           ^

error[E0308]: mismatched types
 --> <anon>:4:5
  |
4 |     s as S.f;
  |     ^^^^^^ expected (), found struct `S`
  |
  = note: expected type `()`
             found type `S`

error: aborting due to previous error
