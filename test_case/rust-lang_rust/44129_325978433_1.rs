
// MIR for `main`
// source = Fn(NodeId(4))
// pass_name = SimplifyCfg-initial
// disambiguator = after

fn main() -> () {
    let mut _0: ();
    let mut _1: D1<'160dce>;
    let mut _2: &'160dce S1;
    let mut _3: &'160dce S1;
    let mut _4: S1;

    bb0: {
        StorageLive(_2);
        StorageLive(_3);
        StorageLive(_4);
        _4 = S1::{{constructor}}(const "tmp",);
        _3 = &'160dce _4;
        _2 = &'160dce (*_3);
        _1 = D1<'160dce>::{{constructor}}(_2,);
        drop(_1) -> bb1;
    }

    bb1: {
        StorageDead(_2);
        StorageDead(_3);
        StorageDead(_4);
        EndRegion('160dce);
        _0 = ();
        return;
    }
}
