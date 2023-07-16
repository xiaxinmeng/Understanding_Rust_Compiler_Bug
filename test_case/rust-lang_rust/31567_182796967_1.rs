
fn(arg0: VecWrapper<'a>) -> &'a u32 {
    let var0: VecWrapper<'a>; // v
    let var1: &'a S; // s_inner
    let mut tmp0: &'a S;
    let mut tmp1: ();
    let mut tmp2: &'a Box<u32>;

    bb0: {
        var0 = arg0;
        tmp0 = &(*(var0.0: &'a mut S)); // borrows *var0.0 for 'a
        var1 = &(*tmp0);
        tmp2 = &((*var1).0: Box<u32>);
        return = &(*(*tmp2));
        drop(var0) -> bb1; // inside 'a, drops var0
    }

    bb1: {
        return;
    }
}
