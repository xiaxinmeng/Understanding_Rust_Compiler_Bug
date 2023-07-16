
rustc 1.17.0 (56124baa9 2017-04-24)
error: `p` does not live long enough
  --> <anon>:14:9
   |
14 |         p.use_up();
   |         ^ does not live long enough
15 |     }
   |     - borrowed value only lives until here
   |
note: borrowed value must be valid for the lifetime 'a as defined on the body at 11:0...
  --> <anon>:11:1
   |
11 |   {
   |  _^ starting here...
12 | |     {
13 | |         let p = P::get();
14 | |         p.use_up();
15 | |     }
16 | | }
   | |_^ ...ending here

error: aborting due to previous error
