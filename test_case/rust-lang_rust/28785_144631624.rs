 rust
fn rangefrom_compare() {
    1.. == (1..); // rustc syntax error, parser-lalr parses as (1..) == (1..)
}
fn compare_rangeto() {
    (..1) == ..1; // rustc syntax error, parser-lalr parses as (..1) == (..1)
}
