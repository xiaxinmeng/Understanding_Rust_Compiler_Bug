
mod foomod {
    pub struct Foo {
        pub foofield: i32,
    }
}

fn main() {
    //    foomod::Foo{f:3i32}
    let t = foomod::Foo{
        foofield: 12,
    };
}
