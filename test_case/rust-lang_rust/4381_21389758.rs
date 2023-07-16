 rust
use std::comm;
fn main() {
    macro_rules! mlet(
        ($($var:ident = $val:expr),*) => (
            let ($($var),*) = ($($val),*);
        );
    );

    let (port, chan) = comm::stream();
    let chan = comm::SharedChan::new(chan);

    mlet!(s1 = chan.clone(), s2 = chan.clone(), s3 = chan.clone());
    do spawn { s1.send( (|a: int| a*2)(10) ) }
    do spawn { s2.send( (|a: int| a*2)(20) ) }
    do spawn { s3.send( (|a: int, b: int| a+b)(30, 40) ) }

    mlet!(x = port.recv(), y = port.recv(), z = port.recv());
    println(fmt!("%d + %d + %d = %d", x, y, z, x + y + z));
}
