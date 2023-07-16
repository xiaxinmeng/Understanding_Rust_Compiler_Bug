 #![feature(plugin)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

fn main() {
    let re = regex!(r"^\d{4}-\d{2}-\d{2}$");
    assert_eq!(re.is_match("2014-01-01"), true);
}
