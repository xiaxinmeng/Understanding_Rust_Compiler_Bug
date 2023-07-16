 rust
fn main() {
    // single sender and receiver
    let (port, chan): (Port<int>, Chan<int>) = Chan::new();
    do spawn {
        chan.send(5);
    }

    println!("{:d}", port.recv());

    // multiple senders
    let (port, chan) = SharedChan::new();

    for i in range(0, 3) {
        let child_chan = chan.clone();
        do spawn {
            child_chan.send(i);
        }
    }

    println!("{:d} {:d} {:d}", port.recv(), port.recv(), port.recv());
}
