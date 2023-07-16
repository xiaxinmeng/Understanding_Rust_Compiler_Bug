rust
use tera::Tera;
use tera::Context;
use serde::Serialize;

#[derive(Serialize)]
struct Settings {
    settings: Vec<Setting>,
}

#[derive(Serialize)]
enum Setting {
    Toggle(bool),
}

fn main() {
    // Use globbing
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let s = Settings {
        settings: vec![Setting :: Toggle(true) ],
    };

    println!("{}", tera.render("settings.html", &Context::from_serialize(s).unwrap()).unwrap());
}
