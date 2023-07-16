 rust

trait FnAlias: Fn() {}

fn main() {
    let _: FnAlias;            // error
    let _: FnAlias<Output=()>; // no error, but I shouldn't be able to do this!
}
