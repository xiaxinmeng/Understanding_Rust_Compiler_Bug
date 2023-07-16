rust
#![feature(proc_macro_hygiene, decl_macro)]

// Either comment this line below with `rocket::post`...
#[rocket::post("/")]
fn problem() -> String {
    // If I surround the lambda with parens as below, everything is okay.
    //
    // if let Some(_) = (|| Some(())) {
    //
    // But if not, the code causing problems is not shown. Also, 
    // instead of a lambda there can be a function call.
    if let Some(_) = || Some(()) {
	return "2".to_string()
    }

    // This should be the line causing the error.
    1
}

fn main() {
}
