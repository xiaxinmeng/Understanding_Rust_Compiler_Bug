rust
enum Either {
    One(X),
    Two(X),
}

struct X;

fn move_into_fnmut() {
    let x = Either::One(X);

    _ = || {
        let Either::Two(a) = x;
        let Either::One(b) = x;
    };
}
