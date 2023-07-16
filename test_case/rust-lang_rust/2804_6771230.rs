

// rustc --lib ice.rs
use std;
import std::map::hashmap;

fn add_interfaces(s: str, data: std::map::hashmap<str, std::json::json>)
{
    #error["Expected list for %s interfaces but found %?", s, data["interfaces"]];
}
