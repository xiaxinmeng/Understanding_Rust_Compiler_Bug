
error: `res` does not live long enough
  --> ./test.rs:17:23
   |
17 |     Ok(try!(do_stuff(&res)))
   |                       ^^^ does not live long enough
18 | }
   | - borrowed value only lives until here
   |
   = note: borrowed value must be valid for the static lifetime...
