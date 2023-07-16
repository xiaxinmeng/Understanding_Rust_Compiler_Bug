rust
type It = u32;

fn iter<'a: 'a>() -> impl Iterator<Item = &'static It> {
    [].into_iter()
}

struct Bivar<'a, I: Iterator<Item = &'a It> + 'a>(I);

fn main() {
    || {
        Bivar(iter())
    };
}
