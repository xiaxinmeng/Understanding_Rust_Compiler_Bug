
mod foo {
    mod bar {
        struct X;
    }
}

use foo::bar::X; // Complains about `bar` being private

fn main() { X }
