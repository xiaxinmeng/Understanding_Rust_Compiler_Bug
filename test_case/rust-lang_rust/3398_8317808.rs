
trait Times2 {
    pure fn times2(it: fn() -> bool);
}


impl i8: Times2 {
    pure fn times2(it: fn() -> bool) {
        let mut i = self;
        while i > 0 {
            if !it() { break }
            i -= 1;
        }
    }
}

impl int: Times2 {
    pure fn times2(it: fn() -> bool) {
        let mut i = self;
        while i > 0 {
            if !it() { break }
            i -= 1;
        }
    }
}

fn main() {
    let mut i = 0;
    for 300.times2 {
        i += 1;
    }

    assert i == 300;
}
