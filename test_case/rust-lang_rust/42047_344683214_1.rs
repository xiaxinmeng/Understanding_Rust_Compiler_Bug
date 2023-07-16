rust
let result = malloc();
let is_ok = foo_abi(&uninit result.repr.ok_payload, &uninit result.repr.err_payload);
result.repr.tag = if is_ok { Ok } else { Err };
