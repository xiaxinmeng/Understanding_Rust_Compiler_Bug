
`Path::exists` only checks whether or not the file was found and readable. By
contrast, `Path::try_exists()` will return `Ok(true)` or `Ok(false)`, respectively, if
the file was verified to exist or verified not to exist, but propegate an Err(_)
if its existance cannot be confirmed or denied. This can be the case if e.g.,
listing permission is denied on one of the parent directories. 
