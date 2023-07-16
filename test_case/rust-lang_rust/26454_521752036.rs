
mod foo {
    mod bar {
        struct X;
    }
}

fn main() { X } // suggests `use foo::bar::X;`
