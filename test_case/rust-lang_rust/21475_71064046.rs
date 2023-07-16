 rust
struct Struct {
    x : u8
}

const C : Struct = Struct { x : 5u8 };

fn main() {
    match 3u8 {
        C.x ... 2u8 => { println!("bad"); }
        2u8 ... C.x => { println!("bad"); }
        _ => {}
    }
}
