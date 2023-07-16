
resource r<T> (_x: T) { io::println("Goodbye, World!") }

fn main() {
    let mut res = r(5);

    let mut v = [mut];
    v <- [mut res];
}
