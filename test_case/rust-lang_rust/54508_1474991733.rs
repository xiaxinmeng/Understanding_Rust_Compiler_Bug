rust
use std::collections::HashSet;

fn callback1(arg1: &str, arg2: &Vec<String>) {}

fn main(){
    let set: HashSet<fn(&str, &Vec<String>)> = HashSet::new();
    HashSet::insert(callback1);
    set.remove(callback1);
}
