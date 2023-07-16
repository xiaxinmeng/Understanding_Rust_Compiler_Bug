
macro m($prefix: ident, $suffix: ident) {
    let a = $prefix::suffix;
    let b = prefix::$suffix;
}
