
rustc 1.19.0-nightly (777ee2079 2017-05-01)
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  --> <anon>:31:14
   |
31 |     r.post_h(move |req: &mut Request| -> IronResult<Response> { body(owned) });
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: the requirement to implement `Fn` derives from here
  --> <anon>:31:7
   |
31 |     r.post_h(move |req: &mut Request| -> IronResult<Response> { body(owned) });
   |       ^^^^^^

error: aborting due to previous error
