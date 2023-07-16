
mod ctr {
    export ctr::{};
    export new; export inc; export print;

    enum ctr { mk_ctr(int) }

    fn new(i: int) -> ctr { mk_ctr(i) }
    fn inc(c: ctr) -> ctr { mk_ctr(*c + 1) }
    fn print(c: ctr) { log(error, *c); }
}


fn main() {
    let c = ctr::new(42);
    ctr::print(c);
    let c2 = ctr::inc(c);
    ctr::print(c2);
    log(error, *c2);
}
