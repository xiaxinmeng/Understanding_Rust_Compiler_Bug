rust
mod generic {
    fn inside_closure<X>(_x: X) { }

    fn foo(a: &mut i32) {
        let _closure = || { inside_closure(a) }; // *moves* `a` due to above signature
    }
}

mod shared_borrow {
    fn inside_closure(_x: &i32) { }

    fn foo(a: &mut i32) {
        {
            println!("we can read a: {:?}", a);
            let _closure = || { inside_closure(a) }; // a shared reborrow; not unique.
            println!("we can read a: {:?}", a);
        }

        *a = 4; // this proves we had shared reborrow for more limited time
    }
}

mod mutate_local {
    fn foo<'r>(mut a: &'r mut i32, b: Option<&'r mut i32>) {
        let _closure = || { a = b.unwrap(); }; // mutates local `a` (via implicit `&mut` borrow)
    }
}
