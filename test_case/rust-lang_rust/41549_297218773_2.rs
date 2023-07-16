
rustc 1.18.0-nightly (63c77214c 2017-04-24)
error[E0326]: implemented const `MY_CONST` has an incompatible type for trait
  --> <anon>:14:21
   |
4  |     const MY_CONST: usize = 4096;
   |                     ----- type in trait
...
14 |     const MY_CONST: u32 = 4096;
   |                     ^^^ expected usize, found u32

error[E0326]: implemented const `MY_OTHER_CONST` has an incompatible type for trait
  --> <anon>:15:27
   |
5  |     const MY_OTHER_CONST: i32 = 16;
   |                           --- type in trait
...
15 |     const MY_OTHER_CONST: u32 = 16;
   |                           ^^^ expected i32, found u32

error: aborting due to 2 previous errors
