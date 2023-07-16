 rust
extern crate syntax;
use syntax::codemap;

fn main() {
    match () {
        codemap::Spanned { node: 0 } => (),
        _ => unreachable!()
    }
}
