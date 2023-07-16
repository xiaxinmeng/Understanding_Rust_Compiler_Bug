
use std;
import std::ptr::addr_of;

type x<T> = {
    a: T,
    b: u8,
    c: bool,
    d: u8,
    e: u8,
    f: u8,
    g: u8
};

fn main() {
    let x: x<int> = {
        a: 12345678,
        b: 9u8,
        c: true,
        d: 10u8,
        e: 11u8,                                                                                                                                                              
        f: 12u8,                                                                                                                                                              
        g: 13u8
    };
    bar(x);
}

fn bar<T>(x: x<T>) {
    log_err addr_of(x.a);
    log_err addr_of(x.b);
    log_err addr_of(x.c);
    log_err addr_of(x.d);
    log_err addr_of(x.e);
    log_err addr_of(x.f);
    log_err addr_of(x.g);
    log_err x.a;
    log_err x.b;
    log_err x.c;
    log_err x.d;
    log_err x.e;
    log_err x.f;
    log_err x.g;
}
