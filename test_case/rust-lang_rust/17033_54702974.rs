 rust
use std::intrinsics::sinf64;

pub type function<T> = fn(x: f64, p: &mut T) -> f64;

struct InternParam<'r, T:'r> {
    func: ::function<T>,
    param: &'r mut T,
    p2: f64
}

struct fn_fourier_params<'r, T:'r> {
    omega: f64,
    function: &'r mut ::function<T>,
    arg: &'r mut T
}

unsafe fn fn_sin<T>(x: f64, p: &mut fn_fourier_params<T>) -> f64 {
    let f = p.function;
    let w = p.omega;
    let wx = w * x;
    let sinwx = sinf64(wx);

    f(x, p.arg) * sinwx
}
