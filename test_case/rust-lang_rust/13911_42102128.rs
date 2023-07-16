 rust
fn connect_into<W: Writer, S: Show, It: Iterator<S>>(w: &mut W, it: It, connector: &str) {
    for (i, x) in it.enumerate() {
       if i == 0 {
           write!(w, "{}", x);
       } else {
            write!(w, "{}{}", connector, x)
       }
    }
}

connect_into(&mut std::io::stdout(), range(0, 3), ", "); // prints: 0, 1, 2
