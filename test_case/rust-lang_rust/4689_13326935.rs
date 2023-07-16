
if compare_and_swap(&mut ptr.unwrapper, 0, serverp) {
    // I get to unwrap the data.
    ...
} else {
    // Multiple unwrappers is illegal.
    die!(...);
}
