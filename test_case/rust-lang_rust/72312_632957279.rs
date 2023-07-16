rust
fn require_static<T: 'static>(val: T) -> T { val }

struct Problem;

impl Problem {
    pub async fn start(&self) {
        require_static(async move {
            &self;
        });
    }
}

fn main() {}
