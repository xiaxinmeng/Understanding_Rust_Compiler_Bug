 rust
fn when(a: Future<A>, b: Future<B>) -> Future <(A, B)> {
    let (port, chan) = oneshot();
    let (port2, chan2) = oneshot();
    let (port_result, chan_result) = oneshot();

    do task:: spawn {
        send_one(chan, a.get_ref());
    }
    do task:: spawn {
        send_one(chan2, b.get_ref());
    };
    do task:: spawn {
        send_one( chan_result
                , (  port.recv(), port2.recv()  ) );
    };

    return from_port(port_result);
}
