rust
{
    let Ok(s) = expr else {
        return 0;
    };
    // many statements
    5
} // end of block

// desugars into
{
    if let Ok(s) = expr {
        // many statements
        5
    } else {
        return 0;
    }
} // end of block
