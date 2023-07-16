rust
struct MustDropFirst;
struct MustDropSecond<'scope>(&'scope MustDropFirst);

impl Drop for MustDropFirst { /* checks drop order between the two types */ }
impl Drop for MustDropSecond { /* checks drop order between the two types */ }

fn main() {
    let data = MustDropFirst;
    let r = &data;
    scope(|s| {
        s.spawn(move || {
            MustDropSecond(r);
        });
    });
}
