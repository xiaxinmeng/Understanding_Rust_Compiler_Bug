rust
mod mutate_deref {
    fn foo(a: &mut i32) { // note: *not* `mut a: &mut i32`
        {
            let _closure = || { *a = 3; }; // mutates deref of `&uniq` borrowed `a`
        }

        *a = 4; // (this proves `a` was reborrowed for limited time, not moved)
    }
}

mod mut_borrow {
    fn inside_closure(_x: &mut i32) { }

    fn foo(a: &mut i32) { // note: *not* `mut a: &mut i32`
        let _closure = || { inside_closure(a) }; // `&uniq` reborrow `a` w/o mutating local
    }
}

mod mutate_field {
    fn foo(a: &mut (i32, i32)) { // note: *not* `mut a`
        {
            let _closure = || { a.0 = 3; }; // mutates deref of `&uniq` borrowed `a`
        }

        a.0 = 4; // (this proves `a` was reborrowed for limited time, not moved)
    }
}
