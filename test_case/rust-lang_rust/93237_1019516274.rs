rust
use std::io::Write;

fn main() {
    let n = 1380;
    let open = "impl Trait<Assoc = ".repeat(n);
    let close = ">".repeat(n);

    let code = format!("
trait Trait {{ type Assoc; }}
impl Trait for () {{ type Assoc = (); }}

fn _f() -> {open}(){close} {{}}

fn main() {{}}
");

    std::fs::File::options()
        .truncate(true)
        .write(true)
        .create(true)
        .open("./nested_rpit.rs")
        .unwrap()
        .write(code.as_bytes())
        .unwrap();   
}
