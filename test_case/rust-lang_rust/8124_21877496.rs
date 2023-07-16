
fn use_immutably(x: &Option<T>, blk: &fn(&T)) { blk(x.get_ref()); } // this borrowchecks today

let x = Some(...);
let y = &mut x;
do use_immutably(&x) |inner_ptr| {
    *y = None; // inner_ptr invalidated
}
