rust
pub fn foo(cmd: &[u8]) -> bool {
    cmd.len() == 3 && cmd.iter().all(|&b| (b as char).is_digit(10))
}

pub fn bar(cmd: &[u8]) -> bool {
    cmd.len() == 3 && cmd.iter().all(|&b| (b as char).is_ascii_digit())
}
