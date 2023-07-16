rust
fn main() {
    let edl = Endless::JustOnce;

    if let Endless::JustOnce = edl {
        println!("Oooh");
    } else {
        println!("Ugh");
    }

    match edl {
        Endless::JustOnce => {},
        _wildcard => {
            // in the future, something will happen here
        }
    }
}
