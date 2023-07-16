 rust
do spawn {
    unsafe {
        let tcp_recv_ptr = tcp_recv_arc.get();
        loop {
            // Then you could use the stream like you would, using (*tcp_recv_ptr)
            let bytes =  (*tcp_recv_ptr).read_to_end();
            // ...
        }
    }
}
