rust
//! ## Sequences
//!
//! |                | get(i)         | insert(i)       | remove(i)      | append | split_off(i)   |
//! |----------------|----------------|-----------------|----------------|--------|----------------|
//! | [`Vec`]        | O(1)           | O(n-i)*         | O(n-i)         | O(m)*  | O(n-i)         |
//! | [`VecDeque`]   | O(1)           | O(min(i, n-i))* | O(min(i, n-i)) | O(m)*  | O(min(i, n-i)) |
//! | [`LinkedList`] | O(min(i, n-i)) | O(min(i, n-i))  | O(min(i, n-i)) | O(1)   | O(min(i, n-i)) |
