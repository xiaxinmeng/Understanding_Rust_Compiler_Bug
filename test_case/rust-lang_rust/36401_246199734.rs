 rust
#[derive(Debug)]
pub enum Event {
    Key(u8),

    Resize,

    Unknown(Vec<u8>),
}

static XTERM_SINGLE_BYTES : [(u8, Event); 1] =
    [ (1,  Event::Resize)
    ];

fn main() {
    println!("{:?}", XTERM_SINGLE_BYTES);
}
