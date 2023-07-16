
use std::slice;

pub fn slice_len(iter: &slice::Iter<u8>) -> usize {
    iter.len()
}

pub fn slice_len2(iter: &slice::Iter<u8>) -> usize {
    iter.size_hint().0
}
