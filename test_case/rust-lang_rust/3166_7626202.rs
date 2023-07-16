
mod argparse {
    use std;

    import std::map;
    import either::{either, left, right};

    struct Flag {
        name: &str;
        desc: &str;
        short_name: option<&str>;
        max_count: uint;
        banner: option<&str>;
        mut value: uint;
    }

    fn flag(name: &str, desc: &str) -> Flag {
        Flag { name: name, desc: desc, short_name: none, max_count: 1, banner: none, value: 0 }
    }

    impl Flag {
        fn short_name(self, s: &self/str) -> Flag/&self {
            let new_banner : option<&str> = match self.banner {
                    none => { none }
                    some(value) => { some(value) }
                };
            Flag {
                name: self.name,
                desc: self.desc,
                short_name: some(s),
                max_count: self.max_count,
                banner: match self.banner {
                        none => { none }
                        some(value) => { some(value) }
                    },
                value: self.value
            }
        }
    }
}

fn main () {
    let f : argparse::Flag = argparse::flag(&"flag", &"My flag");
}
