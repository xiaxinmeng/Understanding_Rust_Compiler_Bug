 rust
struct X {
    a: u8 /// document
    //~^ ERROR found a documentation comment that doesn't document anything
    //~^^ HELP maybe a comment was intended
}
