rust
pub trait MyTrait {
    // @has test_name/trait.MyTrait.html '//[@id='associatedtype.Owned']' "type Gat<'a>"
    type Gat<'a>;
}
