
→ rustc - <<END
#[lit = -1]
struct Foo;
END
error: unexpected token: `-`
 --> <anon>:1:7
  |
1 | #[lit = -1]
  |       ^

error: aborting due to previous error
