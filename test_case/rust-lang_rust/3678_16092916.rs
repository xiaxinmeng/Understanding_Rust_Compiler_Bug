
fn do_spawn_stuff(f: ~fn(&str)) {
    for 5.times {
        do task::spawn {
            f("Stuff");
        }
    }
}
fn main() {
    do_spawn_stuff(|_s| { });
}
