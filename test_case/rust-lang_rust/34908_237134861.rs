 rust
macro_rules! as_item { ($i:item) => { $i } }

macro_rules! foo {
    ($x:tt) => {
        //as_item!{          // uncomment for stable
        struct Item($x);
        //}
    }
}

foo!(i32);

fn main() {}
