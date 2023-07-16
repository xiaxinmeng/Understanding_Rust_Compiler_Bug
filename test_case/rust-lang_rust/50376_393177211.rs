rust
// crate a;
macro m {
    use x::y::z;
}

// crate b;
m!(); // crate_a::x::y::z or crate_b::x::y::z?
