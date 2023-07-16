
fn expect_bound_supply_named<'x>() {
    let mut f: Option<&u32> = None;

    // Here we give a type annotation that `x` should be free. We get
    // an error because of that.
    closure_expecting_bound(|x: &'x u32| {
        //~^ ERROR mismatched types
        //~| ERROR mismatched types

        // And we still cannot let `x` escape into `f`.
        f = Some(x);
        //~^ ERROR cannot infer
    });
}
