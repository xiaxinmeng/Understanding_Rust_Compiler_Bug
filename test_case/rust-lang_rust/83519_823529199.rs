
rustc /tmp/large-stuff.rs '-Zcrate-attr=feature(large_assignments)' '-Zcrate-attr=move_size_limit="1000"'
