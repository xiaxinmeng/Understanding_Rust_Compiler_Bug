 rust
extern mod extra;

use std::cell::Cell;
use extra::arc::MutexArc;
use extra::comm::DuplexStream;

fn main() {
    let arc1 = MutexArc::new(());
    let arc2 = MutexArc::new(());

    let (stream1, stream2) = DuplexStream();

    let arc11 = Cell::new(arc1.clone());
    let arc21 = Cell::new(arc2.clone());
    let stream1 = Cell::new(stream1);
    do spawn {
        let arc1 = arc11.take();
        let arc2 = arc21.take();
        let stream = stream1.take();

        do arc1.access |_| {
            stream.send(());
            stream.recv();
            do arc2.access |_| {}
        }
    }

    let arc12 = Cell::new(arc1);
    let arc22 = Cell::new(arc2);
    let stream2 = Cell::new(stream2);
    do spawn {
        let arc1 = arc12.take();
        let arc2 = arc22.take();
        let stream = stream2.take();

        do arc2.access |_| {
            stream.send(());
            stream.recv();
            do arc1.access |_| {}
        }
    }
}
