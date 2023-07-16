 rust
extern {
    pub static symbol: ();
}
static CRASH: () = symbol;
