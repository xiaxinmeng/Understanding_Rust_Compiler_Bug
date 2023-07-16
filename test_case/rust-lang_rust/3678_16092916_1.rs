
fn do_spawn_stuff(f: extern fn(&str)) {
    for 5.times {
        do task::spawn {
            f("Stuff");
        }
    }
}
fn foo(_s: &str) { }
fn main() {
    do_spawn_stuff(foo);
}
