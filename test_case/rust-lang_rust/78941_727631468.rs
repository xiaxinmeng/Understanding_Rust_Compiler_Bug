
error: functions in `extern` blocks cannot have qualifiers
 --> <anon>:2:11
  |
1 | extern "C" {
  | ---------- in this `extern` block
2 |     async fn calleth();
  |              ^^^^^^^
  |
help: remove the qualifiers
  |
2 |     fn calleth();
  |     ^^

error: aborting due to previous error
