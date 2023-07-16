 rust
extern crate regex;

fn main() {
    let datere = regex::Regex::new(r"^(Mon|Tue|Wed|Thur|Fri|Sat|Sun),\s+(\d+)\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+(\d+:\d+\d+)\s+([+-]\d\d\d\d|\[A-Z]+)$");
    let testdate1 = "Tue, 20 Jan 2015 17:35:20 -0800";
    let result = datere(testdate1); // apply regex
    match result {
        Ok(re) => re,
        Err(err) => panic!("{}", err)
    };
}
