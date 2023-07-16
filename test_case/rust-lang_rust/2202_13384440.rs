
// http://www.paulgraham.com/accgen.html

fn accgen<T:Copy + Num>(n: &a/mut T) -> @a/fn(T) -> T {
    fn@(i: T) -> T { *n += i; *n }
}

fn main() {
    let mut n = 0;
    let f = accgen(&mut n);
    // Should print "1, 2, 3"
    io::println(fmt!("%?, %?, %?", f(1), f(1), f(1)));
}
