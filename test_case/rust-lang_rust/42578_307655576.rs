
error: `<` is interpreted as a start of generic arguments for `usize`, not comparison
 --> <anon>:6:19
  |
6 |     a as usize < b;
  |                ^ <some label, not sure what to write here>
  help: if you want to compare the casted value then write
  |     (a as usize) < b;
