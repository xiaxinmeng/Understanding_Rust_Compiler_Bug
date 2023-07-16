
/* Rust example blurb, with annotated concepts as each is introduced. */
fn main() {
    // basic for loop, optional parentheses, optional argument list
    for 10.times {
        // do notation, module access, task creation
        do task::spawn {
            // type inference, method access, vector literals
            let items = rand::Rng().shuffle([1, 2, 3]);
            // slightly more advanced for loop, macros, format strings, optional semicolons
            for items.each |e| { io::print(fmt!("%d ", e)) }
        }
    }
}
