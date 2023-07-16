 rust
#![feature(macro_rules)]

struct Foo { x: u32, y: u32 }

macro_rules! make_copy(
    ($Self:ident { $($field:ident),+ }) => (
        fn copy(&self) -> $Self {
            //let me = self;
            $Self {
                $($field: self.$field),+
            }
        }
    )
)

impl Foo {
    make_copy!(Foo {x, y})
}

fn main() {
    let x = Foo { x: 0, y: 2 };
    let _y = x.copy();
}
