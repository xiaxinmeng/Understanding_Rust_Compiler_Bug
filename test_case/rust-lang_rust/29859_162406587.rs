
trait Magic: Copy {} impl<T: Magic> Magic for T {}
fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }

#[derive(Debug)]
struct NoClone(String);

fn main() { println!("{:?}", copy(NoClone(String::from("cow")))); }
