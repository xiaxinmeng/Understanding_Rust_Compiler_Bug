
    let vec[int] a = [0];
    auto i = 20;
    auto expected_len = 1u;
    while (i > 0) {
        log_err vec::len(a);
        assert (vec::len(a) == expected_len);
        a += a;
        i -= 1;
        expected_len *= 2u;
    }
