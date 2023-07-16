 rust
static BASE_INSERTION: uint = 32;
static LARGE_INSERTION: uint = 16;

let insertion = if size_of::<T>() <= 16 { BASE_INSERTION } else { LARGE_INSERTION };
