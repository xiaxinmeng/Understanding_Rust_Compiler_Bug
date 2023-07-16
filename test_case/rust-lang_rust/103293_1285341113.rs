rust
struct Thing;

impl Drop for Thing {
    fn drop(&mut self) {}
}

struct Ref<'a>(&'a Thing);

impl<'a> Drop for Ref<'a> {
    fn drop(&mut self) {}
}

fn main() {
    let tmp;
    let _ = match Thing {
        ref thing => {
            tmp = thing;
            true
        }
    } && match Ref(tmp) {
        _ => false,
    };
}
