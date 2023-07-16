rust
match f(unsafe { &mut *value }) {
    Some(value) => Ok(RefMut { value, borrow }),
    None => Err(RefMut { value: unsafe { &mut *value }, borrow }),
}
