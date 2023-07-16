rust
let closure = {
    let capture = String::new();

    move || {
        let _ = capture;
    }
};

let f: fn() = closure;
