rust
macro_rules! tuploop {
    (foreach |$tup:expr, $i:ident: [$($idx:tt),*]| $code:expr) => {
        $(
            let $i = $tup.$idx;
            $code;
        )*
    }
}

fn main() {
    let tup = (1, "hi");
    tuploop! {
        foreach |tup, i: [0, 1]| {
            println!("{}", i);
        }
    }
}
