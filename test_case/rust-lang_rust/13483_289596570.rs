rust
    if condition.is_true() { 
//  ^^ ^^^^^^^^^^^^^^^^^^^ ^
//  1  2                   3
//
// 1 -- we saw this part
// 2 -- this is the expression we expect to parse next
// 3 -- then we expect to see a `{`
}
