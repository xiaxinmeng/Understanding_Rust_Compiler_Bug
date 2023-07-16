
fn main() {
    let (p,c) = pipes::stream();
    do task::spawn |move c| {
        c.send(());
        assert false;
    }
    p.recv();
}
