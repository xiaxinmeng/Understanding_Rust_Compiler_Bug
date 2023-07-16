rust
#![feature(decl_macro)]

fn main() {
    macro foo{
        () => (println!("outer"))
    }
    {
        macro bar{
            () => (foo!())
        }
        {
            macro foo{
                () => (println!("inner"))
            }
            bar!()
        }
    }
}
