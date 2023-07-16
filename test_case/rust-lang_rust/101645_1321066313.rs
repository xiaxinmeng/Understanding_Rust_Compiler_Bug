Rust
if si.hStdInput != 0 || si.hStdOutput != 0 || si.hStdError != 0 {
    si.dwFlags |= c::STARTF_USESTDHANDLES;
}
