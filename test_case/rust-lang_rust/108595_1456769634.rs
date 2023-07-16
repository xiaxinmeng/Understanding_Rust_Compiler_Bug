rust
#![feature(const_fmt_arguments_new)]

const fn my_const_fn() {
    let _  = format_args!("{}", "literal");
    //panic!("{}", "literal");
}

fn main() {
    my_const_fn();
}
