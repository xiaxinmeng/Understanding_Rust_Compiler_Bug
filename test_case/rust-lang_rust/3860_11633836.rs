
struct Parser {
    mut otag: @~str,
}

fn add_tag(self: &Parser) {
    *self.otag + *self.otag;
}

fn main() {}
