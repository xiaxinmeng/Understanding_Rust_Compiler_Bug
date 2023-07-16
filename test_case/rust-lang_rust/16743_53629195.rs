 rust
extern crate time;

use std::io::BufferedReader;
use std::io::File;

fn main() {
    let haystack_path = Path::new("lorem_haystack");
    let haystack_file = File::open(&haystack_path);
    let haystack_string = BufferedReader::new(haystack_file).read_to_string().unwrap();
    let haystack = haystack_string.as_slice();

    let needles_path = Path::new("lorem_needles");
    let needles_file = File::open(&needles_path);
    let needles_string_wrapped = BufferedReader::new(needles_file).read_to_string();
    let needles_string = needles_string_wrapped.unwrap();
    let needles_string_slice = needles_string.as_slice();
    let needles: Vec<&str> = needles_string_slice.split('\n').collect();

    let start = time::precise_time_s();

    let mut cntr = 0 as int;
    for i in range(0u, 100000) {
        for needle in needles.iter() {
            if haystack.contains(*needle) { cntr += 1; }
        }
    }

    let end = time::precise_time_s();

    println!("{}", cntr);
    println!("{}", end - start as f64);
}
