
#![feature(slice_patterns)]

pub struct History<'a> { pub _s: &'a str }

impl<'a> History<'a> {
    pub fn get_page(&self) {
        for s in vec!["1|2".to_string()].into_iter().filter_map(|ref line| self.make_entry(line)) {  
            println!("{:?}", s);
        }
    }

    fn make_entry(&self, s: &'a String) -> Option<&str> {
        let parts: Vec<_> = s.split('|').collect();
        println!("{:?} -> {:?}", s, parts); // (1)

        if let [commit, ..] = &parts[..] { Some(commit) } else { None } //(2)
        //Some(parts[0]) // (3)
    }
}

fn main() {
    let h = History{_s: ""};
    h.get_page();
}
