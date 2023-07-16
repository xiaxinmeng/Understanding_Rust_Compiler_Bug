rust
  // --target i686-unknown-linux-gnu
  // features = ["lua54", "vendored"]
  // overflow-checks = false
  use mlua::{Lua, Function};
  let mut chunk = Vec::new();
  chunk.extend_from_slice(b"\x1bLuaT\0\x19\x93\r\n\x1a\n\x04\x08\x08x");
  chunk.extend_from_slice(b"\x56\0\0\0\0\0\0\0\0\0\0\0(w@\0");
  chunk.extend_from_slice(b"\x80\x80\x80\0\0\0\x01\x7f\x7f\x7f\xff");
  let lua = Lua::new();
  let load: Function<'_> = lua.globals().get("load").unwrap();
  load.call::<_, ()>(lua.create_string(&chunk).unwrap()).unwrap();
  // calls Layout::from_size_align_unchecked(usize::MAX - 3, 8)
  // at mlua::lua::inner_new::allocator()
  