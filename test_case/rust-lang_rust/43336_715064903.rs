Rust
#![feature(try_blocks, never_type, exhaustive_patterns)]

struct Error {
    cause: &'static str,
}

fn oops() -> Result<(), Error> {
    Err(Error { cause: "epic fail" } )
}

fn main() {
    let Err(Error { cause }): Result<!, Error> = try {
        oops()?;
        return // OK
    };
    eprintln!("Nuclear meltdown initiated: {}", cause);
    return ();
}
