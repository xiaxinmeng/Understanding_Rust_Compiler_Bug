
fn main() {
    'a: loop {} // ctxt == 1
    'b: loop {} // ctxt == 2
    'a: loop {} // ctxt == 3
}
