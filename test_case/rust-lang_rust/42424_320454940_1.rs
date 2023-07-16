rust
#![feature(catch_expr)]

fn main() {
    do catch {
        let x: () = Ok(())?;
        Ok(42)
    };
}
