 Rust
fn traverse(l: &mut MyList)
{
    let mut cur = l;
    loop {
        if let &mut MyList::Cons(42, _) = cur {
            break;
        }
        cur = match {cur} {
            &mut MyList::Nil => return,
            &mut MyList::Cons(_, ref mut l) => &mut *l
        }
    }
    // here `l` is active, except for some unknown child of `cur`
    if let &mut MyList::Cons(ref mut a, _) = cur {
        *a += 1;
    }
}
