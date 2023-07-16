 rust
type c_int = ::std::libc::c_int;

mod ntwice {
    type c_int = ::std::libc::c_int;

    #[abi = "cdecl"]
    pub extern {
        fn twice(tf: *u8, x: c_int) -> c_int;
        // a "real" decl for callback
        fn callback(rf: extern "Rust" fn(extern "C" fn(c_int) -> c_int,
                                         x: c_int) -> c_int,
                    x: c_int) -> c_int;
    }
}

extern fn incr2(x:c_int) -> c_int { x+2 }

// <===== no extern decl, unlike `callback` param `rf`.
fn rtwice(f: extern "C" fn(c_int) -> c_int, x: c_int) -> c_int {
    println(fmt!("---- Rust rtwice calling f(%?)", x));
    let ret = f(f(x));
    println(fmt!("---- Rust rtwice calling f(%?) => %?", x, ret));
    ret
}

fn main() {
    println("Hello world");
    unsafe {
        let x = ntwice::twice(incr2, 3);
        println(fmt!("Rust ntwice::twice(incr2, 3): %?", x));
        let y = ntwice::callback(rtwice, 4);
        println(fmt!("Rust ntwice::callback(rtwice, 4): %?", y));
    }
}
