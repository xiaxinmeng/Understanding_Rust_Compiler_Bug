rs
fn foo() {
    fn f() {}
    fn g() {}
    fn g2() -> u8 {
        0
    }
    fn g3(_: u8) {}

    fn h<T>() {}
    fn h2<T>(_: T) {}

    struct Struct();

    let mut x = f;
    x = || ();
    x = (|| ()) as fn();
    x = (|| 0) as fn() -> u8;
    x = (|_| ()) as fn(u8);
    x = g;
    x = g2;
    x = g3;
    x = Struct;

    let mut y = h::<()>;
    y = h::<u8>;

    let mut y2 = h2::<()>;
    y = h2::<u8>;

    let mut z = g3;
    z = (|| ()) as fn();

    let mut p = (|| ()) as fn();
    p = (|| 0) as fn() -> u8;
    p = (|_| ()) as fn(u8);
    let _: fn() -> u8 = g3;
    let _: fn() -> u16 = g2;
}
