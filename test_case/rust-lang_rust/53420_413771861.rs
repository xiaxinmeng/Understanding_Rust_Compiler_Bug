rust
fn main() {
    let v:<() as Lt<'_>>::T = ();
    let f:&mut FnMut<(_,),Output=()> = &mut |_:<() as Lt<'_>>::T|{};
    f(v);
}
