
   Compiling gtk v0.7.0
   Compiling serde v1.0.117
error: could not compile `gio`.

Caused by:
  process didn't exit successfully: `rustc --crate-name gio /home/ellie/.cargo/registry/src/github.com-1ecc6299db9ec823/gio-0.7.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C codegen-units=1 -C debuginfo=2 --cfg 'feature="v2_44"' -C metadata=0564254420026841 -C extra-filename=-0564254420026841 --out-dir /home/ellie/Develop/squeekboard/_build/debug/deps -L dependency=/home/ellie/Develop/squeekboard/_build/debug/deps --extern bitflags=/home/ellie/Develop/squeekboard/_build/debug/deps/libbitflags-14f9ed560b3aed83.rmeta --extern fragile=/home/ellie/Develop/squeekboard/_build/debug/deps/libfragile-5b6284edc93a7050.rmeta --extern gio_sys=/home/ellie/Develop/squeekboard/_build/debug/deps/libgio_sys-14cdd38865daa21e.rmeta --extern glib=/home/ellie/Develop/squeekboard/_build/debug/deps/libglib-878ccd51d1057dc4.rmeta --extern glib_sys=/home/ellie/Develop/squeekboard/_build/debug/deps/libglib_sys-70f0d998180d9afb.rmeta --extern gobject_sys=/home/ellie/Develop/squeekboard/_build/debug/deps/libgobject_sys-90c9207e4cbdf53c.rmeta --extern lazy_static=/home/ellie/Develop/squeekboard/_build/debug/deps/liblazy_static-da3d7afec1ac6b6f.rmeta --extern libc=/home/ellie/Develop/squeekboard/_build/debug/deps/liblibc-ef7e34d7ca0b8bbb.rmeta --cap-lints allow` (signal: 9, SIGKILL: kill)
warning: build failed, waiting for other jobs to finish...
