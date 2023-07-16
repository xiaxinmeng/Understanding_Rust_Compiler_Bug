rust
$ cat /home/joshua/rustc/src/test/rustdoc/assoc-types.rs
// ignore-tidy-linelength

#![crate_type="lib"]

// @has assoc_types/trait.Index.html
pub trait Index<I: ?Sized> {
    // @has - '//*[@id="associatedtype.Output"]//code' 'type Output: ?Sized'
    type Output: ?Sized;
    // @has - '//*[@id="tymethod.index"]//code' \
    //      "fn index<'a>(&'a self, index: I) -> &'a Self::Output"
    // @has - '//*[@id="tymethod.index"]//code//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' \
    //      "Output"
    fn index<'a>(&'a self, index: I) -> &'a Self::Output;
}

// @has assoc_types/fn.use_output.html
// @has - '//*[@class="rust fn"]' '-> &T::Output'
// @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' 'Output'
pub fn use_output<T: Index<usize>>(obj: &T, index: usize) -> &T::Output {
    obj.index(index)
}
