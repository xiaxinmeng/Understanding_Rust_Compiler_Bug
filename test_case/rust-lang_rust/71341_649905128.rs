
error[E0759]: cannot infer an appropriate lifetime
  --> src/main.rs:25:16
   |
22 |     cursor: &'a dyn GeneralControlCursor,
   |             ---------------------------- this data with lifetime `'a`...
...
25 |         cursor.render_frame_container_view().await;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...is captured and required to live as long as `'static` here
