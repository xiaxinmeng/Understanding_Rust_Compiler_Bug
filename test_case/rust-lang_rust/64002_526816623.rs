rust
let mut rule = Rule::new(); 
for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let spt: Vec<&str> = line.split(char::is_whitespace).collect(); 
                                // ^`split(&self, p)` borrows a local resource `line`(it will be dropped when a loop ends).
        match spt[0] {
            "#def" => { rule.name = spt[1]; }
            "#mem" => { rule.mem.push((spt[1], spt[2])) } 
              // you store two references to `line`  into `rule`(declared before this `for` loop),
              // when `line` is dropped at the end of a loop, this two references still live in rule,
              // thus they turn into dangling references as the original resources(line) dropped.
            _ => { println!("{}", line) }
        }
    }
