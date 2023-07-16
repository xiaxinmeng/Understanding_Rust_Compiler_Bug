
/ Why does uncommenting the below item give "error: expected item, found `[`?"
// [#should_panic]
fn test_bad_card() {
    let a = to_card(&"Ten of Spades");
}
