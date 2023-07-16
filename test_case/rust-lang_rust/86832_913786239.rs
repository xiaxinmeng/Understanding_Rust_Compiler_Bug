
$ cargo run --example bufread_reduced_read_count
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/bufread_reduced_read_count`
XOR of all bytes in Cargo.toml is 0x3f
Buffered read was called successfully 209 times
Inner read was called successfully 2 times
$ cargo run --example bufread_assert_stream_position
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/bufread_assert_stream_position`
XOR of all bytes in Cargo.toml is 0x3f
Buffered read was called successfully 209 times
Buffered seek was called successfully 208 times
Inner read was called successfully 209 times
Inner seek was called successfully 208 times
