
fn main() {
    for ["Alice", "Bob", "Carol"].each |n| {
        do task::spawn {
            let v = rand::Rng().shuffle([1, 2, 3]);
            for v.each |e| {
                io::print(fmt!("%s says: '%d'\n", n, e))
            }
        }
    }
}
