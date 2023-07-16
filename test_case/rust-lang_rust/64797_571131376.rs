rust
#[cfg(accessible(peach::Peach))]
mod banana {
    crate struct Banana;
}

#[cfg(accessible(banana::Banana))]
mod peach {
    crate struct Peach;
}
