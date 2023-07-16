
error: lifetime may not live long enough
   --> lib/dal/src/test/helpers.rs:176:5
    |
170 | pub async fn new_ctx_for_new_change_set_and_edit_session<'a, 'b>(
    |                                                          --  -- lifetime `'b` defined here
    |                                                          |
    |                                                          lifetime `'a` defined here
...
176 |     ctx.clone_with_new_visibility(visibility)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
    |
    = help: consider adding the following bound: `'a: 'b`

error: lifetime may not live long enough
   --> lib/dal/src/test/helpers.rs:176:5
    |
170 | pub async fn new_ctx_for_new_change_set_and_edit_session<'a, 'b>(
    |                                                          --  -- lifetime `'b` defined here
    |                                                          |
    |                                                          lifetime `'a` defined here
...
176 |     ctx.clone_with_new_visibility(visibility)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
    |
    = help: consider adding the following bound: `'b: 'a`
