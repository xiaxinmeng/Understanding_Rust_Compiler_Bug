shell
$ cargo new foobar
$ cd foobar
$ cargo build -v
$ ln -s ~/repos/rust/src/etc/lldb_commands ./
$ ln -s ~/repos/rust/src/etc/lldb_providers.py ./
$ ln -s ~/repos/rust/src/etc/lldb_lookup.py ./
$ ln -s ~/repos/rust/src/etc/lldb_batchmode.py ./
$ rust-lldb rust-lldb target/debug/foobar
