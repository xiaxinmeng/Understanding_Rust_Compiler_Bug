
use std::collections::HashMap;

struct something {
    map: HashMap<String, u32>,
}

impl something {
    fn new() -> something {
        return something {
            map: {
                let mut m = HashMap::new();
                m.insert("abcde".to_string(), 3);
                m
            },
        };
    }
}

fn main() {
    let t = something::new();
    let string = "abcde".to_string();

    let x = match t.map.get<S>(&string) {    <--  this invalid <S> is giving the crash in vscode -> output -> rust language server
        Some(z) => (*z).clone(),
        None => 2,
    };
    println!("Hello, world!");
}
