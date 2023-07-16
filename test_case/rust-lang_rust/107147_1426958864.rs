
struct Foo<'a>(&'a [(); 100]);

fn test(x: Foo<'_>) {}

fn main() {}
