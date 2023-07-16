
struct SecretlyEmpty(!);

let res: Result<T, SecretlyEmpty> = ...;
match res {
    Ok(t) => t,
}
