rust
// rustc file.rs -O -Cpasses=lint --crate-type=rlib
pub fn get_physical() {
    get_num_physical_cpus()
}

fn get_num_physical_cpus() {
    use std::collections::HashMap;
    let mut set = HashMap::new();
    set.insert((0, 0), ());
}
