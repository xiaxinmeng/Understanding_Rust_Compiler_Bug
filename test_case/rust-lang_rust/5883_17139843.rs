
struct Struct {
    r: io::Reader
}

fn new_struct(r: io::Reader) -> Struct {
    Struct { r: r }
}
