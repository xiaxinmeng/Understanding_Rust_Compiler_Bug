
error: borrowed value does not live long enough
  --> <anon>:10:19
   |
10 |     let mut err = DB::make().do_thing();
   |                   ^^^^^^^^^^           - temporary value only lives until here
   |                   |
   |                   temporary value created here
11 |     err.done();
12 | }
   | - temporary value needs to live until here
   |
   = note: consider using a `let` binding to increase its lifetime
