rust
#[no_mangle]
pub unsafe extern "C" fn add(argc: u64, mut ap: VaList) -> u64 {
    let mut ret = 0;
    let n = 0;
    while n < argc {
        let tmp = ap.arg::<u64>();
        ret += tmp;
        n += 1;
    }
    ret
}
