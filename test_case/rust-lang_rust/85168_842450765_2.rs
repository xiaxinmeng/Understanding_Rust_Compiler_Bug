rust
use serde_yaml;

fn main() {
    let txt = "---\n- foo\n- bar";
    let word_list: Vec<String> = match serde_yaml::from_str(txt) {
        Ok(lst) => lst,
        serde_yaml::Error(_) => Vec::new(),
    };
    println!("{:?}", word_list);
}
