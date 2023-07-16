rust
enum Nil {
    Nil,
}

struct Cons<T> {
    head: i32,
    tail: T,
}

trait Dot: Sized {
    fn dot(self, other: Self) -> i32;
}

impl Dot for Nil {
    fn dot(self, _: Nil) -> i32 {
        0
    }
}

impl<T: Dot> Dot for Cons<T> {
    fn dot(self, other: Cons<T>) -> i32 {
        self.head * other.head + self.tail.dot(other.tail)
    }
}

fn test<T: Dot>(n: i32, i: i32, first: T, second: T) -> i32 {
    match n {
        0 => first.dot(second),
        _ => test(
            n - 1,
            i + 1,
            Cons {
                head: 2 * i + 1,
                tail: first,
            },
            Cons {
                head: i * i,
                tail: second,
            },
        ),
    }
}

fn main() {
    let n = test(5, 0, Nil::Nil, Nil::Nil);
    println!("{}", n);
}
