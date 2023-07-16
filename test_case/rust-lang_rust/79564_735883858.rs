
error: could not compile `glib`

Caused by:
  process didn't exit successfully: `rustc --crate-name glib src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=0cbf1acb9f52bcf0 -C extra-filename=-0cbf1acb9f52bcf0 --out-dir /tmp/glib/target/release/deps -L dependency=/tmp/glib/target/release/deps --extern bitflags=/tmp/glib/target/release/deps/libbitflags-3481844fb16a27e3.rmeta --extern glib_sys=/tmp/glib/target/release/deps/libglib_sys-805a5615ed4b9e78.rmeta --extern gobject_sys=/tmp/glib/target/release/deps/libgobject_sys-dbf75ab7e9b7edbf.rmeta --extern lazy_static=/tmp/glib/target/release/deps/liblazy_static-53201e5af8cd3135.rmeta --extern libc=/tmp/glib/target/release/deps/liblibc-b4a06f67ef62b8d1.rmeta -C opt-level=2 -C target-cpu=skylake` (signal: 11, SIGSEGV: invalid memory reference)
