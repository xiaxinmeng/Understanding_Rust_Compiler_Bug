
macro_rules! call {
    ($e:expr) => {
        ($e)();
    };
}

fn do_thing() {}

fn main() {
    call!(do_thing());
}
