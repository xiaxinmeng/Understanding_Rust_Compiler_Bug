rust
pub const fn overflow(x: impl Sized) {
    overflow((x,))
}

fn ok() {
    overflow(())
}

pub fn error() {
    overflow(())
}
