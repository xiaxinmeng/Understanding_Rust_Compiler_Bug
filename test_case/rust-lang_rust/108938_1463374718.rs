rust

#[inline(never)]
fn __rust_begin_short_backtrace<T, F: FnOnce() -> T>(f: F) -> T {
    let result = f();

    // prevent this frame from being tail-call optimised away
    std::hint::black_box(result)
}

#[inline(never)]
fn __rust_end_short_backtrace<T, F: FnOnce() -> T>(f: F) -> T {
    let result = f();

    // prevent this frame from being tail-call optimised away
    std::hint::black_box(result)
}


fn first() {
    println!("first ...");
    second();
}

fn second() {
    println!("second ..");
    third();
}

fn third() {
    println!("third ...");
    __rust_end_short_backtrace(|| fourth());
}

fn fourth() {
    println!("fourth ..");
    fifth();
}

fn fifth() {
    println!("fifth ...");
    __rust_begin_short_backtrace(|| sixth());
}

fn sixth() {
    println!("sixth ..");
    seven();
}

fn seven() {
    println!("seven");
    panic!("debug now");
}


fn main() {
    first();
}
