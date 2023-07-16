rust
match errno as c::DWORD {
    c::ERROR_ACCESS_DENIED => return PermissionDenied,
    c::ERROR_ALREADY_EXISTS => return AlreadyExists,
    ...
    _ => {}
}
