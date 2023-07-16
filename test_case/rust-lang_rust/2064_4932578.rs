
unsafe fn buf_init(++input: *u8, len: uint) -> uv_buf_t {
    io::println(#fmt("ll::buf_init - input %u", input as uint));
    let result = rustrt::rust_uv_buf_init(input, len);
    let res_base = get_base_from_buf(result);
    io::println(#fmt("ll::buf_init - result %u", res_base as uint));
    ret result;
}
