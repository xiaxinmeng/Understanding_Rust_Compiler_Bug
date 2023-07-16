
warning: casting function pointer `mem::size_of::<cgmath::Matrix4<f32>>` to `u32`, which truncate
s the value
   --> src\main.rs:413:24
    |
413 |             ByteWidth: mem::size_of::<cgmath::Matrix4<f32>> as _,
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `mem::size_of::
<cgmath::Matrix4<f32>> as usize`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.ht
ml#fn_to_numeric_cast_with_truncation
    = note: `#[warn(clippy::fn_to_numeric_cast_with_truncation)]` on by default
