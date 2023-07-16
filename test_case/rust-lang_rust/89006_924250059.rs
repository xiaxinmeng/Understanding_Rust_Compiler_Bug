rust
const FOO: () = {
    let _ = match SOME_OPERATION {
        Err(_err) => panic!("SOME ERROR"),
        Ok(elem) => elem
    };
};
