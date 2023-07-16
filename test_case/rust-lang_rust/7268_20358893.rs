
/*
error: cannot pack type `~Test`, which does not fulfill `'static`, as a trait bounded by 'static
let trait_obj = (~t) as ~EvalOnce:'static;
                ^~~~~~~~~~~~~~~~~~~~~~~~~
*/

struct Test {
    f: &'static fn:'static(),
}

trait EvalOnce {
    fn eval(~self);
}

impl EvalOnce for Test {
    fn eval(~self) {
        (self.f)();
    }
}

fn test_me() {
    let t: Test  = Test { f: || { } };
    let trait_obj = (~t) as ~EvalOnce:'static;
}
