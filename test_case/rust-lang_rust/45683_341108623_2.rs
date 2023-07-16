rust
let z = {
    let x0 = x.clone();
    move || use(x0) // this is a last-use of `x0`, hence no warning
};
use(x); // uses original `x`
