
error[E0432]: unresolved import `super::gfx`
  --> src/main.rs:10:1
   |
10 | / gfx_defines! {
11 | |     vertex Vertex {
12 | |         pos: [f32; 4] = "a_Pos",
13 | |         color: [f32; 3] = "a_Color",
...  |
24 | |     }
25 | | }
   | |_^ no `gfx` in the root
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
