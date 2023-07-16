
$ ./x86_64-unknown-linux-gnu/stage1/test/stdtest-x86_64-unknown-linux-gnu rt::io

running 81 tests
test rt::io::buffered::test::test_buffered_writer ... ok
task '<unnamed>' failed at 'Unhandled condition: read_error: rt::io::IoError{kind: OtherIoError, desc: "Placeholder error. You shouldn't be seeing this", detail: None}', /home/jason/rust/src/libstd/condition.rs:131
test rt::io::buffered::test::test_buffered_reader ... ok
test rt::io::extensions::test::bytes_0_bytes ... ok
test rt::io::buffered::test::test_buffered_stream ... ok
test rt::io::extensions::test::bytes_eof ... ok
test rt::io::extensions::test::push_bytes ... task 'ok<unnamed>
' failed at 'Unhandled condition: read_error: rt::io::IoError{kind: OtherIoError, desc: "Placeholder error. You shouldn't be seeing this", detail: None}', /home/jason/rust/src/libstd/condition.rs:131
test rt::io::extensions::test::bytes_error ... ok
test rt::io::extensions::test::push_bytes_eof ... ok
test rt::io::extensions::test::push_bytes_error ... ok
test rt::io::extensions::test::push_bytes_partial ... ok
test rt::io::extensions::test::read_byte_0_bytes ... ok
test rt::io::extensions::test::read_byte_eof ... ok
test rt::io::extensions::test::read_byte ... ok
test rt::io::extensions::test::read_bytes ... ok
test rt::io::extensions::test::read_byte_error ... ok
test rt::io::extensions::test::read_bytes_eof ... ok
test rt::io::extensions::test::read_bytes_partial ... ok
test rt::io::extensions::test::read_to_end ... ok
test rt::io::extensions::test::push_bytes_fail_reset_len ... ok
test rt::io::extensions::test::test_read_be_int_n ... ok
test rt::io::extensions::test::test_read_f32 ... ok
test rt::io::extensions::test::test_read_write_be ... ok
test rt::io::extensions::test::read_to_end_error ... ok
test rt::io::extensions::test::test_read_write_f32 ... ok
test rt::io::extensions::test::test_read_write_le_mem ... ok
test rt::io::flate::test::smoke_test ... ignored
test rt::io::mem::test::test_buf_reader ... ok
test rt::io::mem::test::test_mem_reader ... ok
test rt::io::mem::test::test_mem_writer ... ok
test rt::io::mem::test::test_with_mem_writer ... ok
test rt::io::net::ip::test::ipv6_addr_to_str ... ok
test rt::io::net::ip::test::test_from_str_ipv4 ... ok
test rt::io::net::ip::test::test_from_str_ipv4_in_ipv6 ... ok
test rt::io::net::ip::test::test_from_str_ipv6 ... ok
test rt::io::net::ip::test::test_from_str_socket_addr ... ok
test rt::io::net::tcp::test::bind_error ... ignored
failed in non-task context at 'assertion failed: handle.is_not_null()', /home/jason/rust/src/libstd/rt/uv/mod.rs:105
failed in non-task context at 'assertion failed: handle.is_not_null()', /home/jason/rust/src/libstd/rt/uv/mod.rs:105
failed in non-task context at 'assertion failed: handle.is_not_null()', /home/jason/rust/src/libstd/rt/uv/mod.rs:105
failed in non-task context at 'assertion failed: handle.is_not_null()', /home/jason/rust/src/libstd/rt/uv/mod.rs:105
failed in non-task context at 'assertion failed: handle.is_not_null()', /home/jason/rust/src/libstd/rt/uv/mod.rs:105
Aborted
