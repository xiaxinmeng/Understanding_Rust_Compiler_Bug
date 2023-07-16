
resource r<T> (x: T) { fail }

enum e<T: const> { e(r<T>) }

fn foo() -> e<int> {fail;}

fn main() {
    let x = foo();
}
