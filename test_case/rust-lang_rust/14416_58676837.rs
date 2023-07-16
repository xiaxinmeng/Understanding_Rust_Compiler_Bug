
let (hash, rlib) = if file.starts_with(rlib_prefix.as_slice()) &&
        file.ends_with(".rlib") {
    (file.slice(rlib_prefix.len(), file.len() - ".rlib".len()),
     true)
} else if dypair.map_or(false, |(_, suffix)| {
    file.starts_with(dylib_prefix.get_ref().as_slice()) &&
    file.ends_with(suffix)
}) {
    let (_, suffix) = dypair.unwrap();
    let dylib_prefix = dylib_prefix.get_ref().as_slice();
    (file.slice(dylib_prefix.len(), file.len() - suffix.len()),
     false)
} else {
    return FileDoesntMatch
};
