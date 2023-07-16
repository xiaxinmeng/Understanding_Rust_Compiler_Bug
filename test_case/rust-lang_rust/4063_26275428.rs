 rust
enum State { ST_NULL = 0, ST_WHITESPACE=1 }

fn SomeFunction () {
    ~[ST_NULL, ..(ST_WHITESPACE as uint)];
}

fn main() {}
