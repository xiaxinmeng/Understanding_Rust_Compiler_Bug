rust
let capture;

let closure = move || {
    let _ = capture; //~ error: use of possibly-uninitialized variable: `capture`
};
capture = 0;
