rust
#![feature(extended_key_value_attributes)]

const _: &[u8] = compile_time_run::run_command!("sh", "-c", "echo '    .second(\"hello\")' > somefile");

/// You are supposed to call it like this
///
/// # Example
///
/// 