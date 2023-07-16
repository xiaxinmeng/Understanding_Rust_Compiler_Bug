rust
let ip : Ipv4Addr = ...;

match ip {
  Ipv4Addr::LOCALHOST => { ... },
  _ => { ... }
}
