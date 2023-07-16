 rust
cond! {
    x < 0 { io::println(~"woops");   }
    true  { io::println(~"hullo");   }
    _     { io::println(~"bananas"); }
}
