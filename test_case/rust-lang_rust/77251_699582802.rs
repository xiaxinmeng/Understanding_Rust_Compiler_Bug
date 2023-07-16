rust
struct Log { s: String }
const LOG: Log = Log { s: String::new() };

fn main() {
    LOG.s = "~~~".to_owned(); //~attempting to modify a `const` item
}
