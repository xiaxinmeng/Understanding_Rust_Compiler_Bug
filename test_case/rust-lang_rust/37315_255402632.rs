 rust
fn select_fold1<I,B, FProj, FCmp>(it: I,
                                  mut f_proj: FProj,
                                  mut f_cmp: FCmp) -> Option<(B, I::Item)>
    where I: Iterator,
          FProj: FnMut(&I::Item) -> B,
          FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
{
    let mut projected = it.map(|x| (f_proj(&x), x));
    projected.next().map(|(sel_p, sel)| {
        projected.fold((sel_p, sel), |(sel_p, sel), (x_p, x)| {
            if f_cmp(&sel_p, &sel, &x_p, &x) {
                (x_p, x)
            } else {
                (sel_p, sel)
            }
        })
    })
}


