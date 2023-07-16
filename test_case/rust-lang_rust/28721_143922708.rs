 rust
struct Message<'a> {
    data: &'a u32,
}

// The second for<> is written out for clarity, but could be elided.
struct Handler<for<'a> T<'a>>(Option<Box<for<'a> Fn(T<'a>)>>);

struct Cfg {
    handler: Handler<Message>,
}
