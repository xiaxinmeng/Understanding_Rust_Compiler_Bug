rust
// traits mixed-up slightly, check links for actual requirements

// https://en.cppreference.com/w/cpp/named_req/Iterator
unsafe trait CppIterator: Clone + Default {
    type Item;
    /// Safety: must not be past-the-end or otherwise invalid
    /// Equivalent to C++'s `++iter`
    unsafe fn move_to_next(&mut self);
    // omitted `iter++`
    /// Safety: must be past-the-end or before-begin or otherwise invalid
    /// Equivalent to `*iter` and `iter.operator ->()`
    unsafe fn get(&self) -> *mut Self::Item;
}

unsafe impl<T> CppIterator for *mut T {
    type Item = T;
    unsafe fn move_to_next(&mut self) {
        *self = self.add(1);
    }
    unsafe fn get(&self) -> *mut Self::Item {
        *self
    }
}

// https://en.cppreference.com/w/cpp/named_req/InputIterator
unsafe trait InputIterator: PartialEq + CppIterator {}

unsafe impl<T> InputIterator for *mut T {}

// https://en.cppreference.com/w/cpp/named_req/ForwardIterator
unsafe trait ForwardIterator: InputIterator {}

unsafe impl<T> ForwardIterator for *mut T {}

// https://en.cppreference.com/w/cpp/named_req/BidirectionalIterator
unsafe trait BidirectionalIterator: ForwardIterator {
    /// Safety: must not be before-begin or otherwise invalid
    /// Equivalent to C++'s `--iter`
    unsafe fn move_to_prev(&mut self);
    // omitted `iter--`
}

unsafe impl<T> BidirectionalIterator for *mut T {}

// https://en.cppreference.com/w/cpp/iterator/begin
// https://en.cppreference.com/w/cpp/iterator/end
unsafe trait BeginAndEnd {
    type Iter: CppIterator;
    /// return an iterator that is either past-the-end (in case of an empty container) or at the beginning
    unsafe fn begin(self: *mut Self) -> Self::Iter;
    /// return a past-the-end iterator
    unsafe fn end(self: *mut Self) -> Self::Iter;
}

unsafe impl<T> BeginAndEnd for [T] {
    type Iter = *mut T;
    unsafe fn begin(self: *mut Self) -> Self::Iter {
        self as *mut T
    }
    unsafe fn end(self: *mut Self) -> Self::Iter {
        let len = (*self).len();
        (self as *mut T).add(len)
    }
}
