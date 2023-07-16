
error: expected one of `!`, `(`, `+`, `,`, `::`, `<`, or `>`, found `;`
 --> <anon>:6:19
  |
6 |     a as usize < b;
  |                   ^ expected one of 7 possible tokens here
  help: may be you want to compare the casted value?
  |     (a as usize) < b;
