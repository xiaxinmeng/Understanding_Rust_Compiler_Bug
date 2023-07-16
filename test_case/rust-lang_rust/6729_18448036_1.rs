 rust
let ret1 = do "function foo (x,y) return x+y end".as_c_str |s| {
    luaL_loadstring(L, s);
}
