rust
let result_code = try {
    write!(...)?;
    0
} catch {
    Err(e) => 1,
};
