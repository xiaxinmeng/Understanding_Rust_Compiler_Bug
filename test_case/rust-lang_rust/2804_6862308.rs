 rust
use std;
import std::map::hashmap;

fn add_interfaces(managed_ip: str, device: std::map::hashmap<str, std::json::json>)  {
    log(error, #fmt("%s, %?", managed_ip, device["interfaces"]));
}
