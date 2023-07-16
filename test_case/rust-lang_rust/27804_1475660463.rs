rust
/// Pops the element immediately before the specified value
pub fn pop_before<K: Ord>(set: &mut BTreeSet<K>, value: &K) -> Option<K>
{
    let key_ref = {
        if let Some(key_ref) = set.range(..value).next_back() {
            /* must hide the origin of this borrow ... */
            unsafe { &*(key_ref as *const _) }
        } else {
            return None;
        }
    };
    /* ... so that we may be able to mutably borrow the set here
    despite key_ref existence */
    set.take(key_ref)
}

/// Pops the element immediately after the specified value
pub fn pop_after<K: Ord>(set: &mut BTreeSet<K>, value: &K) -> Option<K>
{
    let key_ref = {
        if let Some(key_ref) = set.range(value..).next() {
            /* must hide the origin of this borrow ... */
            unsafe { &*(key_ref as *const _) }
        } else {
            return None;
        }
    };
    /* ... so that we may be able to mutably borrow the set here
    despite key_ref existence */
    set.take(key_ref)
}

/// Pops the element equal to the specified value
pub fn pop<K>(set: &mut BTreeSet<K>, value: &K) -> Option<K>
where
    K: Ord,
{
    let key_ref = {
        if let Some(key_ref) = set.range(..=value).next() {
            /* must hide the origin of this borrow ... */
            unsafe { &*(key_ref as *const _) }
        } else {
            return None;
        }
    };
    /* ... so that we may be able to mutably borrow the set here
    despite key_ref existence */
    set.take(key_ref)
}
