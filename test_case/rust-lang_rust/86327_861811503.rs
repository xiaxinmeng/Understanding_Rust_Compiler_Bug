rust
extern "C" {
    // @has 'foo/fn.needs_drop.html'
    // @has - '//pre[@class="rust fn"]' 'pub unsafe extern "C" fn needs_drop() -> !'
    pub fn needs_drop() -> !;
}
