
fn main() {
    let functions = [
        || io::println("LOL FIRST"),
        || io::println("LOL SECOND")
    ];

    for functions.each |func| {
        (*func)();
    }
}
