rust
loop {
    {
        if let $p:pattern = $ei:expr {
            break $em:expr
        }
    }
    break $ef:expr
}
