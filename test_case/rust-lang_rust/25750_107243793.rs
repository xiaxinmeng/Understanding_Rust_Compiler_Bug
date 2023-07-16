 rust
# Fails
fn fail() -> Option<(Box<Fn(&Sized)>, Box<Fn(&Sized)>)> {
    None
}

# Works
fn work<'a>() -> Option<(Box<Fn(&'a Sized)>, Box<Fn(&'a Sized)>)> {
    None
}
