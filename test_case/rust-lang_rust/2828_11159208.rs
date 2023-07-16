
struct NoCopy {
    n: int
}
fn NoCopy() -> NoCopy {
    NoCopy { n: 0 }
}

impl NoCopy: Drop {
    fn finalize(&self) {
        log(error, "running destructor");
    }
}

fn main() {
    let x = NoCopy();

    let f = fn~(move x) { assert x.n == 0; }; 
    let g = copy f; // <-- Error expected here

    f(); g();
}
