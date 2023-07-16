rust
#![feature(proc_macro_hygiene, decl_macro)]

type Result<T> = std::result::Result<T, ()>;

pub fn f<T>(f: &dyn Fn() -> Result<T>) -> Result<T> {
    f()
}

// Either comment this line below with `rocket::post`...
#[rocket::post("/")]
fn open_session() -> Result<String> {
    // ...or comment this if, or don't call "f" function
    if let Ok(_) = f(&|| Ok(())) {
    // This if also reproduces.
    // if let Ok(_) = &|| Ok(()) {
	return Ok("2".to_string())
    }

    Ok("1")
}

fn main() {
}
