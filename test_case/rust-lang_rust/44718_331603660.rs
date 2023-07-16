rust
#[get("/")]
fn handler<'r>() -> impl Responder<'r> { ... }
