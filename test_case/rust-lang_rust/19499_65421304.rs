 rust

fn main() {
    let n = 0;
    let it = Some(1u).into_iter().inspect2(|_| {n;});
    //~^ error: unable to infer enough type information about `Inspect<uint, core::option::Item<uint>, closure>`; type annotations required

    // Trigger
    // let _: uint = n;  // Ok
    it.count();
    let _: uint = n;  // Err
}
