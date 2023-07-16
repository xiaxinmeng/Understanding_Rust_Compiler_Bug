
rustc examples/draw_state_test.rs
--crate-name draw_state_test
--crate-type bin
-g
--out-dir /Users/sven/rust/gfx_graphics/target/debug/examples
--emit=dep-info,link
-L dependency=/Users/sven/rust/gfx_graphics/target/debug
-L dependency=/Users/sven/rust/gfx_graphics/target/debug/deps
--extern freetype=/Users/sven/rust/gfx_graphics/target/debug/deps/libfreetype-3fe4d3cf608e3e5b.rlib 
--extern sdl2_window=/Users/sven/rust/gfx_graphics/target/debug/deps/libsdl2_window-729940b7a5ce5bbd.rlib
--extern gfx_texture=/Users/sven/rust/gfx_graphics/target/debug/deps/libgfx_texture-80eaa31d63c63e75.rlib
--extern piston=/Users/sven/rust/gfx_graphics/target/debug/deps/libpiston-1c842b7b5fd7b8b8.rlib 
--extern image=/Users/sven/rust/gfx_graphics/target/debug/deps/libimage-85c07ef1cf5314da.rlib 
--extern graphics=/Users/sven/rust/gfx_graphics/target/debug/deps/libgraphics-3ec65f4f2805722e.rlib 
--extern gfx_macros=/Users/sven/rust/gfx_graphics/target/debug/deps/libgfx_macros-996769dbcfbdc64a.dylib
--extern gfx=/Users/sven/rust/gfx_graphics/target/debug/deps/libgfx-9b45fdf5b2760af1.rlib
--extern gfx_device_gl=/Users/sven/rust/gfx_graphics/target/debug/deps/libgfx_device_gl-f20b4ab1c3bd6bf0.rlib
--extern gfx_graphics=/Users/sven/rust/gfx_graphics/target/debug/libgfx_graphics-577c6c6700ce1009.rlib
