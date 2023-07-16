rust
#![feature(slice_patterns)]

fn main() {
    #[derive(Debug)]
    enum E<X> { A(X), B { x: X } }

    #[derive(Debug)]
    struct S<'a, X: 'a, Y>{ x: &'a [X], y: (Y, Y), };

    let e = E::A(3);

    match e {
        E::A(ref ax) => println!("e.ax: {:?}", ax),
        E::B { x: ref bx } => println!("e.bx: {:?}", bx),
    }

    let s = S { x: &[1, 2, 3,], y: (999, 998) };

    match s {
        S  {
            x: &[ref x0, ref xn..],
            y: (ref y0, ref y1)
        } => {
            println!("s.x0: {:?} s.xn: {:?}", x0, xn);
            println!("s.y0: {:?}, s.y1: {:?}", y0, y1);
        }

        _ => panic!("other case"),
    }
}
