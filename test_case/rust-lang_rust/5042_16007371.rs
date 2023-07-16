 rust
fn main() -> () {
    let atom1 = atomic_init(&0);

    for [0, ..5].each |_| {
        do task::spawn || {
            for [0, ..5].each |_| {
                atom1.add(2);
                io::println(fmt!("[%d] Var is %d", *(task::get_task()), atom1.load());
            }
        }
    }
}
