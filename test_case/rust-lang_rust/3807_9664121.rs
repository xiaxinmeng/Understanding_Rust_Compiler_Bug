
fn main() {
    let v = ~[1, 2, 3];
    loop foo: {
        for v.each |e| { break foo; }
        io::println("Hello, world!\n");
        break;
    }
    io::println("Goodbye, world!\n");
}
