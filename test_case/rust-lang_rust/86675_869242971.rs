rust
 use macros::demo_proc_macro;

macro_rules! normal_macro {
    ($($input:tt)*) => {
        let _ = move | a : String | demo_proc_macro!();
    } ;
}

fn main() {
    let a = "some string".to_owned();

    normal_macro!();

    let _ = a.clone();
}
