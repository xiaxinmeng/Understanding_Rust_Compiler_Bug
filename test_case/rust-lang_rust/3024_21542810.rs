
struct dummy {
    x: int
}

impl Drop for dummy {
    fn drop(&self) {
        println(fmt!("Hi: %d", self.x));
    }
}

fn welp<T>(a: ~T) -> T {
    let ~x = a;
    x
}

fn main() {
    let o = ~dummy { x: 5 };
    welp(o);
}
