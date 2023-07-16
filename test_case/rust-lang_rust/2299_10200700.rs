
struct Bird {
    x: int
}

trait Chirpy {
    fn tweet();
}

impl Bird: Chirpy {
    fn tweet() {}
}

fn main() {
    let x = @Bird{x: 5};
    let y = x as @Chirpy;
}
