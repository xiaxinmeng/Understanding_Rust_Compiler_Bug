
error[E0759]: `cursor` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
  --> src/main.rs:25:16
   |
22 |     cursor: &'a dyn GeneralControlCursor,
   |             ---------------------------- this data with lifetime `'a`...
...
25 |         cursor.render_frame_container_view().await;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...is captured and required to live as long as `'static` here
   |
note: the used `impl` has a `'static` requirement
  --> src/main.rs:13:44
   |
13 | impl GeneralControlCursorRendering for dyn GeneralControlCursor {
   |                                            ^^^^^^^^^^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
14 |     fn render_frame_container_view<'a>(
   |        --------------------------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
13 | impl GeneralControlCursorRendering for dyn GeneralControlCursor + '_ {
   |                                                                 ^^^^
