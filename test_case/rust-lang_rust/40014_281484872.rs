
error[E0101]: cannot infer the return type of this closure
 --> <anon>:2:17
  |
2 |     let _v = || [];
  |                 ^^ cannot infer the return type of this closure
  suggestion: add an explicit annotation like `|| -> [XXX; 0] { [] }`
