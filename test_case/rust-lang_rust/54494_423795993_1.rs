rust
let fd = {
    let temp;
    {
        temp = file_result.unwrap();
        temp.as_raw_fd();
    }
}
