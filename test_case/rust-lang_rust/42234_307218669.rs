rust
fn foo() -> Result<(), ()> {
    let x = None;
    Ok(x.unwrap().make()?)
}

fn main() { }
