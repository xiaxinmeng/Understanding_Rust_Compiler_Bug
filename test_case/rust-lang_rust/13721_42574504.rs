 rust
#[deriving(Show)]
struct MyStruct {
    x: int
    #[deriving_show="p.display()"]
    p: Path
    y: int
}
