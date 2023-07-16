rust
fn main() {
     ::std::process::exit(match run_your_app() {
        Ok(0) => 1, // no matches
        Ok(_) => 0, // some matches
        Err(someerr) => {
           report(someerr); 2 // return whatever you want depending on the error
        }
     });
}
