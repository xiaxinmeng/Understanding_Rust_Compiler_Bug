rust
#[macro_use]
extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Rust Rocket is running!\n"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
