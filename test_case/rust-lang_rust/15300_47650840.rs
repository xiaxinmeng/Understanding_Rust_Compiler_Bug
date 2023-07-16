 rust
trait Actually {
    fn actually<U: ::std::fmt::Show>(self, U) -> U;
}

impl<T: ::std::fmt::Show> Actually for T {
    fn actually<U: ::std::fmt::Show>(self, u: U) -> U {
        println!("{} actually {}", self, u);
        u
    }
}

fn main() {
    let mut a;
    // -13, 5, -13, 5, -23, 5, -23, 5, -8, 5.
    println!("{}", {a = 2i; a} - {a = 3; a} * {a = 5; a});
    println!("{}", a);
    println!("{}", (a = 2i).actually(a) - (a = 3).actually(a) * (a = 5).actually(a));
    println!("{}", a);
    println!("{}", {a = 2i; a} - a * {a = 5; a});
    println!("{}", a);
    println!("{}", (a = 2i).actually(a) - a * (a = 5).actually(a));
    println!("{}", a);
    println!("{}", (a = 2i).actually(a) - ().actually(a) * (a = 5).actually(a));
    println!("{}", a);

    let mut a;
    // These ones always end up as `false`
    println!("{}", {a = true; a} && {a = false; a});
    println!("{}", a);
    println!("{}", {a = false; a} && {a = true; a});
    println!("{}", a);
    println!("{}", (a = true).actually(a) && (a = false).actually(a));
    println!("{}", a);
    println!("{}", (a = false).actually(a) && (a = true).actually(a));
    println!("{}", a);

    // These ones always end up as `true`
    println!("{}", {a = true; a} || {a = false; a});
    println!("{}", a);
    println!("{}", {a = false; a} || {a = true; a});
    println!("{}", a);
    println!("{}", (a = true).actually(a) || (a = false).actually(a));
    println!("{}", a);
    println!("{}", (a = false).actually(a) || (a = true).actually(a));
    println!("{}", a);
}
