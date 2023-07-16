rust
#[link(name = "hello", kind = "static", modifiers = "+whole-archive")]
extern "C" {
    fn hello();
}

#[link(name = "hello", kind = "static", modifiers = "-whole-archive")]
extern "C" {}
