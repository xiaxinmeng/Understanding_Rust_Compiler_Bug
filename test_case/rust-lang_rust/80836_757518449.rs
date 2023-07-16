
if head != tail { // is_empty()
    if idx < head.wrapping_sub(tail) & size { // idx < count()
        return Some();
    } else {
        return None;
    }
}
