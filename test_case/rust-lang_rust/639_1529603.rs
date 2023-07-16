
iter under(uint hi) -> uint {
    let uint y = 0u;
    while (y < hi) {
        put y;
        y += 1u;
    }
}

iter several_zeroes() -> uint {
    for each (uint i in under(3u)) {
        put 0u;
    }
}
