
$ git grep "transmute::<&"
src/libstd/sys/unix/process.rs:            mem::transmute::<&ProcessConfig<K,V>,&'static ProcessConfig<K,V>>(cfg)
src/libsyntax/ast.rs:            ::std::mem::transmute::<&str,&str>(&token::get_name(*self))
src/libsyntax/parse/token.rs:            mem::transmute::<&[u8],&[u8]>(this.container_as_bytes())
src/test/run-make/mixing-deps/prog.rs:    assert_eq!(unsafe { mem::transmute::<&int, uint>(&both::foo) },

