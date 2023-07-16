
[01:00:35] test net::tcp::tests::bind_error ... ok
[01:00:35] thread '<unnamed>' panicked at 'received error for `TcpListener::bind(&addr)`: Cannot assign requested address (os error 99)', /checkout/src/libstd/net/tcp.rs:1433:20
[01:00:35] test net::tcp::tests::clone_accept_concurrent ... FAILED
[01:00:35] thread '<unnamed>' panicked at 'received error for `TcpListener::bind(&addr)`: Cannot assign requested address (os error 99)', /checkout/src/libstd/net/tcp.rs:1415:20
...
[01:00:47] failures:
[01:00:47]     net::tcp::tests::clone_accept_concurrent
[01:00:47]     net::tcp::tests::clone_accept_smoke
[01:00:47]     net::tcp::tests::clone_while_reading
[01:00:47]     net::tcp::tests::close_read_wakes_up
[01:00:47]     net::tcp::tests::close_readwrite_smoke
[01:00:47]     net::tcp::tests::connect_loopback
[01:00:47]     net::tcp::tests::double_bind
[01:00:47]     net::tcp::tests::fast_rebind
[01:00:47]     net::tcp::tests::multiple_connect_interleaved_greedy_schedule
[01:00:47]     net::tcp::tests::multiple_connect_interleaved_lazy_schedule
[01:00:47]     net::tcp::tests::multiple_connect_serial
[01:00:47]     net::tcp::tests::partial_read
[01:00:47]     net::tcp::tests::peek
[01:00:47]     net::tcp::tests::read_eof
[01:00:47]     net::tcp::tests::shutdown_smoke
[01:00:47]     net::tcp::tests::smoke_test
[01:00:47]     net::tcp::tests::socket_and_peer_name
[01:00:47]     net::tcp::tests::tcp_clone_smoke
[01:00:47]     net::tcp::tests::tcp_clone_two_read
[01:00:47]     net::tcp::tests::tcp_clone_two_write
[01:00:47]     net::tcp::tests::write_close
[01:00:47]     net::udp::tests::connect_send_peek_recv
[01:00:47]     net::udp::tests::peek_from
[01:00:47]     net::udp::tests::set_nonblocking
[01:00:47]     net::udp::tests::socket_name_ip4
[01:00:47]     net::udp::tests::socket_smoke_test_ip4
[01:00:47]     net::udp::tests::udp_clone_smoke
[01:00:47]     net::udp::tests::udp_clone_two_read
[01:00:47]     net::udp::tests::udp_clone_two_write
[01:00:47] 
[01:00:47] test result: FAILED. 773 passed; 29 failed; 0 ignored; 0 measured; 0 filtered out
