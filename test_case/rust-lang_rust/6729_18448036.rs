 rust
let ret1 = str::as_c_str("function foo (x,y) return x+y end",
                         |s| luaL_loadstring(L, s));
