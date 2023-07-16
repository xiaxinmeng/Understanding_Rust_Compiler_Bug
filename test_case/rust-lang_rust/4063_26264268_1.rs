
enum State { ST_NULL, ST_WHITESPACE }

fn SomeFunction () {
    ~[ST_NULL, ..(ST_WHITESPACE as uint)];
}

fn main() {}
