rust
#[derive(PartialEq, Debug)]
enum Res<'a, T> {
    Ok(&'a str, T),
    Err(String)
}

fn get_res<'a, T>(s: &'a str) -> Res<'a, T> {
    Ok(s, 0)
}

macro_rules! is_err { ($e:expr) => (match $e { Err(_) => true, Ok(_, _) => false})}

fn main() {
    println!("{}", is_err!(get_res()));
}
