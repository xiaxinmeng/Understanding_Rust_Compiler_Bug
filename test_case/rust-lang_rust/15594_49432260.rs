 rust
#![no_std]
#![feature(lang_items, macro_rules)]

// Pull in the system libc library for what crt0.o likely requires
extern crate libc;

pub enum Option<T> { None, Some(T) }

#[inline(never)]
fn f(x: uint) -> uint { x + 1 }
macro_rules! create_exprs {
    ($e: expr; ) => { ::f(::f($e) + ::f($e)) };
    ($e: expr; $_x: expr $($xs: expr)*) => {
        {
            let a = create_exprs!($e; $($xs)*);
            let b = create_exprs!($e; $($xs)*);
            let c = create_exprs!($e; $($xs)*);
            let d = create_exprs!($e; $($xs)*);
            let e = create_exprs!($e; $($xs)*);
            let f = create_exprs!($e; $($xs)*);
            let g = create_exprs!($e; $($xs)*);
            let h = create_exprs!($e; $($xs)*);
            if a + b == 0 {
                c + d
            } else if e + f == 0 {
                g
            } else {
                h
            }
        }
    }
}

macro_rules! create {
    ($i: item; ) => { $i };
    ($i: item; $_x: expr $($xs: expr)*) => {
        mod foo { create!($i; $($xs)*) }
        mod bar { create!($i; $($xs)*) }

        pub fn baz(x: ::Option<uint>) -> ::Option<uint> {
            use None;
            let z = match x {
                ::Some(y) if y > 10 => create_exprs!(y; 2 16 128),
                ::Some(z) => create_exprs!(z; 2 16 128),
                None => create_exprs!(0u; 2 16 128)
            };
            bar::baz(bar::baz(foo::baz(foo::baz(::Some(z)))))
        }
    }
}

create!(pub fn baz(x: ::Option<uint>) -> ::Option<uint> {
    #![inline(never)]
    use None;
    match x { ::Some(a) => ::Some(a + 1), None => None }
}; 1 2 4 8 16 32)

// Entry point for this program
#[start]
fn start(_argc: int, _argv: *const *const u8) -> int {
    match bar::baz(foo::baz(foo::baz(None))) {
        Some(x) => x as int,
        None => 0
    }
}

// These functions are invoked by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
