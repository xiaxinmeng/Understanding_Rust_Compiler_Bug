rust
const fn make_options() {
    // These already work because they are special cased:
    Some(0);
    (Option::Some)(1);
    // These also work now:
    let f = Option::Some;
    f(2);
    {Option::Some}(3);
}
